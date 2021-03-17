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
};
use frame_system::{
	ensure_signed,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

pub const GUESS_NUMBERS_COUNT: usize = 6;

type SessionIdType = u128;
type BetType = u32;
type GuessNumbersType = [u8; GUESS_NUMBERS_COUNT];

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Bet<AccountId> {
	account_id: AccountId,
	guess_numbers: GuessNumbersType,
	bet: BetType,
}

decl_storage! {
	trait Store for Module<T: Config> as JackBlock {
		SessionId get(fn session_id): SessionIdType;
		Bets get(fn bets): map hasher(blake2_128_concat) SessionIdType => Vec<Bet<T::AccountId>>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		NewBet(SessionIdType, Bet<AccountId>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		ExampleError,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

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
	}
}