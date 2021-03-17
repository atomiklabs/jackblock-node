use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(JackBlock::example_extrinsic(Origin::signed(1)));
	});
}