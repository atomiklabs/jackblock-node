use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn add_new_bet_works() {
	new_test_ext().execute_with(|| {
		let session_id = JackBlock::session_id();
		assert_eq!(JackBlock::bets(session_id), vec![]);

		let (account_id, guess_numbers, bet) = (1, [0; crate::GUESS_NUMBERS_COUNT], 100);
		
		assert_ok!(JackBlock::add_new_bet(Origin::signed(account_id), guess_numbers, bet));
		
		let bet = crate::Bet {
			account_id,
			guess_numbers,
			bet,
		};

		assert_eq!(JackBlock::bets(session_id), vec![bet]);
	});
}