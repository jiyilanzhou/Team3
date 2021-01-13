use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*};
use crate::RawEvent;

#[test]
fn test_create(){
	new_test_ext().execute_with( || {
		run_to_block(10);
		assert_eq!(Kitties::create(Origin::signed(1)), Ok(()));
		assert_eq!(last_event(), RawEvent::Created(1,0));
	})
}

#[test]
fn test_transfer_kitties() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(Kitties::create(Origin::signed(1)));
		let id = Kitties::kitties_count();

		assert_ok!(Kitties::transfer(Origin::signed(1), 2 , id-1));
		assert_eq!(last_event(), RawEvent::Transferred(1,2, id-1));
		assert_noop!(
                Kitties::transfer(Origin::signed(1), 2, id-1),
                Error::<Test>::NotKittyOwner
                );
	})
}

#[test]
fn test_breed() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(
                Kitties::breed(Origin::signed(1), 0, 1),
                Error::<Test>::InvalidKittyId
                );

		assert_ok!(Kitties::create(Origin::signed(1)));
		assert_ok!(Kitties::create(Origin::signed(2)));
		assert_ok!(Kitties::breed(Origin::signed(1),0,1));

		assert_eq!(last_event(), RawEvent::Created(1, 2));
	})
}