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
		Currency,
		ExistenceRequirement::KeepAlive,
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
	ModuleId,
	RandomNumberGenerator,
	traits::{
		BlakeTwo256,
		IdentifyAccount,
		AccountIdConversion,
		Saturating,
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
use sp_arithmetic::Percent;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod group_by;
pub use group_by::{GroupByTrait};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"jack");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{app_crypto::{app_crypto, sr25519}, MultiSigner, MultiSignature};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

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
	type Currency: Currency<Self::AccountId>;
}
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const BET_PRICE: u32 = 1000000000; // TODO: u128
const SESSION_IN_BLOCKS: u8 = 5;
const MIN_GUESS_NUMBER: u32 = 1;
const MAX_GUESS_NUMBER: u32 = 10;
const GUESS_NUMBERS_COUNT: usize = 6;
const UNSIGNED_TX_PRIORITY: u64 = 100;
const PALLET_ID: ModuleId = ModuleId(*b"JackPot!");

type SessionIdType = u128;
type GuessNumbersType = [u8; GUESS_NUMBERS_COUNT];
type Winners<AccountId> = Vec<(Bet<AccountId>, u8)>;
type NFTHash = Vec<u8>;


#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, PartialOrd, Ord)]
pub struct NFTRequestData<AccountId, Balance> {
	winner_account: AccountId,
	reward: Balance,
	score: u8,
	score_out_of: u8,
	session_id: SessionIdType,
}

type NFTRequestDataOf<T> = NFTRequestData<<T as frame_system::Config>::AccountId, BalanceOf<T>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Bet<AccountId> {
	account_id: AccountId,
	guess_numbers: GuessNumbersType,
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct NftHashPayload<Public> {
	public: Public,
	nft_hash: NFTHash,
}

impl<T: SigningTypes> SignedPayload<T> for NftHashPayload<T::Public> {
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
		PendingWinnersNFT get(fn pending_winners_nft): Vec<NFTRequestDataOf<T>>;

		Authorities get(fn authorities) config(offchain_authorities): Vec<T::AccountId>;
	}
}

decl_event!(
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		Balance = BalanceOf<T>,
		{
		NewBet(SessionIdType, Bet<AccountId>),
		Winners(SessionIdType, Winners<AccountId>),
		SessionResults(SessionIdType, GuessNumbersType, Winners<AccountId>),
		RewardFeeForAuthority(AccountId, Balance),
		RewardForWinner(AccountId, Balance),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		SessionIdOverflow,
		TryToFinalizeTheSessionWhichIsNotClosed,
		PendingWinnerDoesNotExist,
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
			// TODO - set offchain worker lock to do not start twice for the same session

			if let Some(session_id) = Self::closed_not_finalised_session() {
				if let Err(error) = Self::generate_session_numbers_and_send(block_number, session_id) {
					debug::RuntimeLogger::init();
					debug::info!("--- offchain_worker error: {}", error);
				}
			}
			
			// TODO - set offchain worker lock to do not start twice for the same PendingWinnersNFT
			let pending_winners_nft = Self::pending_winners_nft();
			if pending_winners_nft.len() > 0 {
				Self::generafte_pending_winners_nft(pending_winners_nft);
			}
		}

		#[weight = 10_000]
		pub fn add_new_bet(origin, guess_numbers: GuessNumbersType) {
			let account_id = ensure_signed(origin)?;
			let session_id = SessionId::get();

			let new_bet = Bet {
				account_id: account_id.clone(),
				guess_numbers,
			};

			let bet_price: BalanceOf<T> = BET_PRICE.into(); // TODO: impl _u128.into()

			Bets::<T>::try_mutate(session_id, |bets| -> DispatchResult {
				T::Currency::transfer(&account_id, &Self::account_id(), bet_price, KeepAlive)?;
				bets.push(new_bet.clone());
				Ok(())
			})?;

			Self::deposit_event(RawEvent::NewBet(session_id, new_bet));
		}

		#[weight = 10_000]
		pub fn add_nft_hash_to_winner(origin, nft_request_data: NFTRequestDataOf<T>, payload: NftHashPayload<T::Public>, _singature: T::Signature) {
			ensure_none(origin)?;

			let mut pending_winners = Self::pending_winners_nft();

			match pending_winners.binary_search(&nft_request_data) {
				Ok(index) => {
					pending_winners.remove(index);
					PendingWinnersNFT::<T>::put(pending_winners);
				},
				Err(_) => {
					return Err(Error::<T>::PendingWinnerDoesNotExist.into())
				},
			};

			debug::RuntimeLogger::init();
			debug::info!("--- add_nft_hash_to_winner: account_id: {:?} / nft_hash: {:?}", nft_request_data.winner_account, payload.nft_hash);
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

			Self::deposit_event(RawEvent::SessionResults(payload.session_id, payload.session_numbers, winners.clone()));

			debug::RuntimeLogger::init();
			debug::info!("--- Finalize_the_session: {}", payload.session_id);
			debug::info!("--- Session_numbers: {:?}", payload.session_numbers);
			debug::info!("--- Winners: {:?}", winners);

			if winners.len() > 0 {
				let (_, pot) = Self::pot();
				let fees = Percent::from_percent(10) * pot;
				let pot_for_rewards = pot.saturating_sub(fees);
				let authorities = Self::authorities();
				let authorities_count = authorities.len() as u32;
				let reward_fee_per_authority: BalanceOf<T> = fees / authorities_count.into(); // TODO: fixed point safe division

				debug::info!("--- Pot before: {:?}", pot);
				debug::info!("--- Pot for fees: {:?} $", fees);
				debug::info!("--- Pot for rewards: {:?} $", pot_for_rewards);

				for authoritiy in authorities {
					debug::info!("--- Reward for authority: {:?}, {:?} $", authoritiy, reward_fee_per_authority);
					T::Currency::transfer(&Self::account_id(), &authoritiy, reward_fee_per_authority, KeepAlive)?;
					Self::deposit_event(RawEvent::RewardFeeForAuthority(authoritiy, reward_fee_per_authority));
				};

				let winners_to_reward: Winners<T::AccountId> = winners.into_iter().filter(|&(_, hits) | hits >= 3).collect();
				let winners_grouped_by_hits = winners_to_reward.group_by(|(_, a_hits), (_, b_hits)| a_hits == b_hits);

				winners_grouped_by_hits.for_each(|winners| {
					let hits = winners[0].1;

					match hits {
						3 => {
							Self::distribute_reward(3, payload.session_id, winners, pot_for_rewards, hits);
						},
						4 => {
							Self::distribute_reward(7, payload.session_id, winners, pot_for_rewards, hits);
						},
						5 => {
							Self::distribute_reward(15, payload.session_id, winners, pot_for_rewards, hits);
						},
						6 => {
							Self::distribute_reward(75, payload.session_id, winners, pot_for_rewards, hits);
						},
						_ => debug::info!("Error distribute_reward"), // TODO: handle Error
					}
				});

				let (_, pot) = Self::pot();
				debug::info!("--- Pot after: {:?} $", pot);
			}
		}
	}
}

impl<T: Config> Module<T> {
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	fn pot() -> (T::AccountId, BalanceOf<T>) {
			let account_id = Self::account_id();
			let balance = T::Currency::free_balance(&account_id)
				.saturating_sub(T::Currency::minimum_balance());

			(account_id, balance)
	}

	fn distribute_reward(reward_percentage: u8, session_id: SessionIdType, winners: &[(Bet<T::AccountId>, u8)], pot_for_rewards: BalanceOf<T>, hits: u8) {
		let rewards_from_pot = Percent::from_percent(reward_percentage) * pot_for_rewards;
		let winners_count = winners.len() as u32;
		let reward_per_winner: BalanceOf<T> = rewards_from_pot / winners_count.into(); // TODO: fixed point safe division

		winners.iter().for_each(|winner| {
			let winner_account = &winner.0.account_id;
			debug::info!("Account {:?} won {:?} $ by guessing {:?} numbers!", winner_account, reward_per_winner, hits);
			let _ = T::Currency::transfer(&Self::account_id(), winner_account, reward_per_winner, KeepAlive); // TODO: handle erorr

			PendingWinnersNFT::<T>::mutate(|pending_winners_nft| {
				let nft_request_data = NFTRequestData {
					winner_account: winner_account.clone(),
					reward: reward_per_winner,
					score: hits,
					score_out_of: GUESS_NUMBERS_COUNT as u8,
					session_id,
				};

				pending_winners_nft.push(nft_request_data);
			});

			Self::deposit_event(RawEvent::RewardForWinner(winner_account.clone(), reward_per_winner));
		})
	}

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

	fn is_authority_account(account_id: &T::AccountId) -> bool {
		Self::authorities().contains(account_id)
	}

	#[cfg(test)]
	fn set_session_id(session_id: SessionIdType) {
		SessionId::put(session_id);
	}



	// --- Off-chain workers ------------------------

	fn generate_session_numbers_and_send(block_number: T::BlockNumber, session_id: SessionIdType) -> Result<(), &'static str> {
		let session_numbers = Self::get_session_numbers();

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

	fn generafte_pending_winners_nft(nft_requests_data: Vec<NFTRequestDataOf<T>>) {
		nft_requests_data.iter().for_each(|request_data| {
			debug::RuntimeLogger::init();
			debug::info!("--- offchain_worker generafte_pending_winners_nft: {:?}", request_data);

			let nft_hash: NFTHash = Vec::new();

			let _result = Self::winner_nft_hash_send_unsigned(request_data.clone(), nft_hash); // TODO - Handle result errors
		});
	}

	fn winner_nft_hash_send_unsigned(nft_request_data: NFTRequestDataOf<T>, nft_hash: NFTHash) -> Result<(), &'static str> {
		let (_account, result) = Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
			|account| NftHashPayload {
				public: account.public.clone(),
				nft_hash: nft_hash.clone(),
			},
			|payload, signature| {
				Call::add_nft_hash_to_winner(nft_request_data.clone(), payload, signature)
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

				// TODO - use aura keys to check authority

				let account_id = payload.public.clone().into_account();
				if !Self::is_authority_account(&account_id) {
					return InvalidTransaction::BadProof.into();
				}

				return ValidTransaction::with_tag_prefix("JackBlock/validate_unsigned/finalize_the_session")
					.priority(UNSIGNED_TX_PRIORITY)
					.longevity(5)
					.propagate(true)
					.build();
			},
			Call::add_nft_hash_to_winner(_ntf_request_data, ref payload, ref signature) => {
				let valid_signature = SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !valid_signature {
					return InvalidTransaction::BadProof.into();
				}

				// TODO - Refactor this method to be used in finalize_the_session & add_nft_hash_to_winner
				// TODO - use aura keys to check authority

				let account_id = payload.public.clone().into_account();
				if !Self::is_authority_account(&account_id) {
					return InvalidTransaction::BadProof.into();
				}

				return ValidTransaction::with_tag_prefix("JackBlock/validate_unsigned/add_nft_hash_to_winner")
					.priority(UNSIGNED_TX_PRIORITY)
					.longevity(5)
					.propagate(true)
					.build();
			}
			_ => return InvalidTransaction::Call.into(),
		};
	}
}
