use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};

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

#[test]
fn next_session_id_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(JackBlock::session_id(), 0);
		
		assert_ok!(JackBlock::next_session_id());
		assert_eq!(JackBlock::session_id(), 1);
		
		JackBlock::set_session_id(crate::SessionIdType::MAX);
		assert_eq!(JackBlock::session_id(), crate::SessionIdType::MAX);
		
		assert_noop!(JackBlock::next_session_id(), crate::Error::<Test>::SessionIdOverflow);
	});
}

#[test]
fn get_winners_works() {
	new_test_ext().execute_with(|| {
		let session_numbers = [5, 21, 48, 25, 34, 18];

		let session_bets = vec!(
			crate::Bet {
				account_id: 1,
				guess_numbers: [33, 48, 18, 2, 5, 8],
				bet: 100,
			},
			crate::Bet {
				account_id: 2,
				guess_numbers: [9, 3, 1, 21, 43, 4],
				bet: 200,
			},
			crate::Bet {
				account_id: 3,
				guess_numbers: [8, 8, 8, 8, 8, 8],
				bet: 300,
			},
			crate::Bet {
				account_id: 4,
				guess_numbers: [42, 29, 8, 1, 5, 18],
				bet: 400,
			},
		);

		let expected_result = vec!(
			(session_bets[0].clone(), 3),
			(session_bets[1].clone(), 1),
			(session_bets[3].clone(), 2),
		);

		assert_eq!(JackBlock::get_winners(session_numbers, session_bets), expected_result);
	});
}