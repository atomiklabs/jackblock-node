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
	},
	debug,
	unsigned::{
		ValidateUnsigned,
	},
};
use frame_system::{
	ensure_signed,
	offchain::{
		AppCrypto,
		CreateSignedTransaction,
		SignedPayload,
		SigningTypes,
		Signer,
		SendUnsignedTransaction,
	}
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
	},
};
use sp_io::{
	offchain,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
}

const SESSION_IN_BLOCKS: u8 = 10;
const MIN_GUESS_NUMBER: u32 = 1;
const MAX_GUESS_NUMBER: u32 = 49;
const GUESS_NUMBERS_COUNT: usize = 6;

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
		
		ClosedNotFinalisedSessions get(fn closed_not_finalised_sessions): Vec<SessionIdType>;
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

			if Self::closed_not_finalised_sessions().is_empty() {
				return
			}

			let session_id = Self::closed_not_finalised_sessions().pop().unwrap(); // TODO - replace unwrap()

			Self::generate_session_numers_and_send(block_number, session_id); // TODO - handle errors
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
		pub fn finalize_the_session(origin, payload: SessionNumbersPayload<T::Public, T::BlockNumber>, _singature: T::Signature) -> Result<(), DispatchError> {
			// ensure signed by off-chain worker

			let SessionNumbersPayload {session_id, session_numbers, public, block_number} = payload;

			let session_bets = Bets::<T>::get(session_id);
			
			let winners = Self::get_winners(session_numbers, session_bets);

			// remove session_id from ClosedNotFinalisedSessions
			
			debug::info!("--- finalize_the_session: {}", session_id);
			debug::info!("--- session_numbers: {:?}", session_numbers);
			debug::info!("--- winners: {:?}", winners);
	
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	fn close_the_session() -> Result<(), DispatchError> {
		let session_id = Self::next_session_id()?;
		ClosedNotFinalisedSessions::mutate(|x| x.push(session_id));
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

	fn generate_session_numers_and_send(block_number: T::BlockNumber, session_id: SessionIdType) -> Result<(), &'static str> {
		let session_numbers = Self::get_session_numbers();

		debug::warn!("--- off-chain session_numbers: {:?}", session_numbers);

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
			_ => return InvalidTransaction::Call.into(),
		};
	}
}