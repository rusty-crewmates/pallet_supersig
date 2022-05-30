use crate::Error;
use super::mock::*;
use super::helper::*;
use frame_support::{
	assert_noop, assert_ok,
	traits::{Currency, ReservableCurrency},
};
pub use sp_std::boxed::Box;

#[test]
fn remove_supersig() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(Supersig::create_supersig(
			Origin::signed(ALICE()),
			vec!(ALICE(), BOB(), CHARLIE()),
		));
		let supersig_id = get_account_id(0);
		let bob_balance = Balances::free_balance(BOB());
		let amount = 10_000u64;
		assert_ok!(Balances::transfer(
			Origin::signed(ALICE()),
			supersig_id.clone(),
			amount
		));
		assert_ok!(Supersig::remove_supersig(
			Origin::signed(supersig_id.clone()),
			supersig_id.clone(),
			BOB()
		));

		assert_eq!(Supersig::supersigs(0), None);
		assert_eq!(Supersig::nonce_call(0), 0);
		assert!(Supersig::calls(0, 0).is_none());
		assert_eq!(Supersig::votes(0, 0), 0);
		assert_eq!(frame_system::Pallet::<Test>::providers(&supersig_id), 0);
		assert_eq!(
			Balances::free_balance(BOB()),
			bob_balance + amount + Balances::minimum_balance()
		);
	})
}

#[test]
fn remove_supersig_not_allowed() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(Supersig::create_supersig(
			Origin::signed(ALICE()),
			vec!(ALICE(), BOB()),
		));
		let supersig_id = get_account_id(0);
		assert_noop!(
			Supersig::remove_supersig(Origin::signed(ALICE()), supersig_id, BOB()),
			Error::<Test>::NotAllowed
		);
	})
}

#[test]
fn remove_supersig_unknown_supersig() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(Supersig::create_supersig(
			Origin::signed(ALICE()),
			vec!(ALICE(), BOB()),
		));
		let bad_supersig_id = get_account_id(1);
		assert_noop!(
			Supersig::remove_supersig(
				Origin::signed(bad_supersig_id.clone()),
				bad_supersig_id,
				BOB()
			),
			Error::<Test>::SupersigNotFound
		);
	})
}

#[test]
fn cannot_remove_supersig() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(Supersig::create_supersig(
			Origin::signed(ALICE()),
			vec!(ALICE(), BOB(), CHARLIE()),
		));
		let supersig_id = get_account_id(0);
		let amount = 10_000u64;
		assert_ok!(Balances::transfer(
			Origin::signed(ALICE()),
			supersig_id.clone(),
			amount
		));
		assert_ok!(Balances::reserve(&supersig_id, amount));
		assert_noop!(
			Supersig::remove_supersig(Origin::signed(supersig_id.clone()), supersig_id, BOB()),
			Error::<Test>::CannotDeleteSupersig
		);
	})
}

#[test]
fn cannot_liquidate_supersig() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_ok!(Supersig::create_supersig(
			Origin::signed(ALICE()),
			vec!(ALICE(), BOB(), CHARLIE()),
		));
		let supersig_id = get_account_id(0);

		let call = Call::Balances(pallet_balances::Call::transfer_all {
			dest: ALICE(),
			keep_alive: false,
		});

		assert_ok!(Supersig::submit_call(
			Origin::signed(ALICE()),
			supersig_id.clone(),
			Box::new(call.clone())
		));

		assert_ok!(Supersig::approve_call(
			Origin::signed(BOB()),
			supersig_id.clone(),
			0
		));

		assert_ok!(Supersig::approve_call(
			Origin::signed(CHARLIE()),
			supersig_id.clone(),
			0
		));

		assert!(Supersig::calls(0, 0).is_none());

		assert!(System::account_exists(&supersig_id));
	});
}

