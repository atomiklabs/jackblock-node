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
};
use frame_system::{
	ensure_signed,
};
use sp_runtime::{
	RandomNumberGenerator,
	traits::{
		BlakeTwo256,
	},
};
use sp_io::{
	offchain,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

const SESSION_IN_BLOCKS: u8 = 10;
const MIN_GUESS_NUMBER: u32 = 1;
const MAX_GUESS_NUMBER: u32 = 49;
const GUESS_NUMBERS_COUNT: usize = 6;

type SessionIdType = u128;
type BetType = u32;
type GuessNumbersType = [u8; GUESS_NUMBERS_COUNT];
type Winners<AccountId> = Vec<(Bet<AccountId>, u8)>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Bet<AccountId> {
	account_id: AccountId,
	guess_numbers: GuessNumbersType,
	bet: BetType,
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

			Self::generate_session_numers_and_sign();
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
		pub fn finalize_the_session(origin, session_id: SessionIdType, session_numbers: GuessNumbersType) -> Result<(), DispatchError> {
			// ensure signed by off-chain worker

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

	fn generate_session_numers_and_sign() {
		let session_numbers = Self::get_session_numbers();

		debug::warn!("--- off-chain session_numbers: {:?}", session_numbers);
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