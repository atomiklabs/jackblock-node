#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module,
	decl_storage,
	decl_event,
	decl_error,
	codec::{
		Encode,
		Decode,
	},
	traits::{
		Vec,
	},
	dispatch::{
		DispatchError,
		DispatchResult,
	},
	debug,
	unsigned::{
		ValidateUnsigned,
	},
};
use frame_system::{
	ensure_signed,
	ensure_none,
	offchain::{
		AppCrypto,
		CreateSignedTransaction,
		SignedPayload,
		SigningTypes,
		Signer,
		SendUnsignedTransaction,
	},
};
use sp_runtime::{
	RandomNumberGenerator,
	traits::{
		BlakeTwo256,
	},
	RuntimeDebug,
	transaction_validity::{
		TransactionSource,
		TransactionValidity,
		InvalidTransaction,
		ValidTransaction,
	},
};
use sp_io::{
	offchain,
};
use sp_core::{
	crypto::{
		KeyTypeId,
	},
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"jack");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSigner,
		MultiSignature,
	};
	use sp_core::sr25519::Signature as Sr25519Signature;
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	type Call: From<Call<Self>>;
}

const SESSION_IN_BLOCKS: u8 = 5;
const MIN_GUESS_NUMBER: u32 = 1;
const MAX_GUESS_NUMBER: u32 = 49;
const GUESS_NUMBERS_COUNT: usize = 6;
const UNSIGNED_TX_PRIORITY: u64 = 100;

type SessionIdType = u128;
type BetType = u32;
type GuessNumbersType = [u8; GUESS_NUMBERS_COUNT];
type Winners<AccountId> = Vec<(Bet<AccountId>, u8)>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Bet<AccountId> {
	account_id: AccountId,
	guess_numbers: GuessNumbersType,
	bet: BetType,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct SessionNumbersPayload<Public, BlockNumber> {
	public: Public,
	block_number: BlockNumber,
	session_id: SessionIdType,
	session_numbers: GuessNumbersType,
}

impl<T: SigningTypes> SignedPayload<T> for SessionNumbersPayload<T::Public, T::BlockNumber> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}

decl_storage! {
	trait Store for Module<T: Config> as JackBlock {
		SessionId get(fn session_id): SessionIdType;
		SessionLength: T::BlockNumber = T::BlockNumber::from(SESSION_IN_BLOCKS);
		Bets get(fn bets): map hasher(blake2_128_concat) SessionIdType => Vec<Bet<T::AccountId>>;
		ClosedNotFinalisedSessionId get(fn closed_not_finalised_session): Option<SessionIdType>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		NewBet(SessionIdType, Bet<AccountId>),
		Winners(SessionIdType, Winners<AccountId>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		SessionIdOverflow,
		TryToFinalizeTheSessionWhichIsNotClosed,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		fn on_finalize(block_number: T::BlockNumber) {
			if block_number % SessionLength::<T>::get() == T::BlockNumber::from(0u8) {
				let _ = Self::close_the_session();
			}
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("--- offchain_worker: {:?}", block_number);

			if let Some(session_id) = Self::closed_not_finalised_session() {
				if let Err(error) = Self::generate_session_numbers_and_send(block_number, session_id) {
					debug::info!("--- generate_session_numbers_and_send error: {}", error);
				}
			}
		}

		#[weight = 10_000]
		pub fn add_new_bet(origin, guess_numbers: GuessNumbersType, bet: BetType) {
			let account_id = ensure_signed(origin)?;
			let session_id = SessionId::get();

			let new_bet = Bet {
				account_id,
				guess_numbers,
				bet,
			};

			Bets::<T>::mutate(session_id, |bets| bets.push(new_bet.clone()));

			Self::deposit_event(RawEvent::NewBet(session_id, new_bet));
		}

		#[weight = 10_000]
		pub fn finalize_the_session(origin, payload: SessionNumbersPayload<T::Public, T::BlockNumber>, _singature: T::Signature) {
			ensure_none(origin)?;

			ClosedNotFinalisedSessionId::try_mutate(|x| -> DispatchResult {
				match x {
					None => return Err(Error::<T>::TryToFinalizeTheSessionWhichIsNotClosed)?,
					Some(value) => {
						if *value != payload.session_id {
							return Err(Error::<T>::TryToFinalizeTheSessionWhichIsNotClosed)?
						}

						*x = None;
						return Ok(())
					},
				};
			})?;

			let session_bets = Bets::<T>::get(payload.session_id);
			let winners = Self::get_winners(payload.session_numbers, session_bets);
			
			debug::info!("--- finalize_the_session: {}", payload.session_id);
			debug::info!("--- session_numbers: {:?}", payload.session_numbers);
			debug::info!("--- winners: {:?}", winners);
		}
	}
}

impl<T: Config> Module<T> {
	fn close_the_session() -> DispatchResult {
		let session_id = Self::next_session_id()?;
		ClosedNotFinalisedSessionId::put(session_id);
		Ok(())
	}

	fn get_winners(session_numbers: GuessNumbersType, session_bets: Vec<Bet<T::AccountId>>) -> Winners<T::AccountId> {
		session_bets.into_iter()
			.map(|bet| {
				let correct = session_numbers.iter()
					.filter(|n| bet.guess_numbers.contains(n))
					.fold(0, |acc, _| acc + 1);
			
				(bet, correct)
			})
			.filter(|x| x.1 > 0)
			.collect::<Winners<T::AccountId>>()
	}

	fn next_session_id() -> Result<SessionIdType, DispatchError> {
		let session_id = SessionId::get();
		let next_session_id = session_id.checked_add(1).ok_or(Error::<T>::SessionIdOverflow)?;
		SessionId::put(next_session_id);

		Ok(session_id)
	}

	#[cfg(test)]
	fn set_session_id(session_id: SessionIdType) {
		SessionId::put(session_id);
	}



	// --- Off-chain workers ------------------------

	fn generate_session_numbers_and_send(block_number: T::BlockNumber, session_id: SessionIdType) -> Result<(), &'static str> {
		let session_numbers = Self::get_session_numbers();

		debug::info!("--- off-chain session_numbers: {:?}", session_numbers);

		let (_account, result) = Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
			|account| SessionNumbersPayload {
				public: account.public.clone(),
				block_number,
				session_id,
				session_numbers,
			},
			|payload, signature| {
				Call::finalize_the_session(payload, signature)
			}
		).ok_or("No local accounts accounts available")?;

		result.map_err(|()| "Unable to submit transaction")?;

		Ok(())
	}

	fn get_random_number() -> u8 {
		let random_seed = offchain::random_seed();
		let mut rng = RandomNumberGenerator::<BlakeTwo256>::new(random_seed.into());

		(rng.pick_u32(MAX_GUESS_NUMBER - MIN_GUESS_NUMBER) + MIN_GUESS_NUMBER) as u8
	}

	fn get_session_numbers() -> GuessNumbersType {
		let mut session_numbers: GuessNumbersType = [0; GUESS_NUMBERS_COUNT];

		let mut i = 0;
		loop {
			let next_session_number = Self::get_random_number();
			if !session_numbers.contains(&next_session_number) {
				session_numbers[i] = next_session_number;
			    i += 1;
			}
			
			if i == GUESS_NUMBERS_COUNT {
			    break;
			}
		}

		session_numbers
	}
}

impl<T: Config> ValidateUnsigned for Module<T> {
	type Call = Call<T>;

	fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		match call {
			Call::finalize_the_session(ref payload, ref signature) => {
				let valid_signature = SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !valid_signature {
					return InvalidTransaction::BadProof.into();
				}

				// TODO - ensure that was sent from off-chain worker

				return ValidTransaction::with_tag_prefix("JackBlock/validate_unsigned/finalize_the_session")
					.priority(UNSIGNED_TX_PRIORITY)
					.longevity(5)
					.propagate(true)
					.build();
			},
			_ => return InvalidTransaction::Call.into(),
		};
	}
}