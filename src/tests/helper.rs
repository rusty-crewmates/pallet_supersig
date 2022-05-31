use super::mock::*;
use sp_runtime::traits::AccountIdConversion;

pub fn get_account_id(index: u64) -> <Test as frame_system::Config>::AccountId {
	SupersigPalletId::get().into_sub_account(index)
}