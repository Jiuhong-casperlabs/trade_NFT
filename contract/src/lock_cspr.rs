#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, URef, U512};

pub const GROUP_LABEL: &str = "group_label";
pub const GROUP_UREF_NAME: &str = "group_uref";

#[repr(u16)]
enum Error {
    UserInsuffucientbalance = 2,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");
    let contract_hashs: Key = runtime::get_named_arg("contract_hash");
    let target: Key = runtime::get_named_arg("target");
    let contract_hash: ContractHash;
    match contract_hashs {
        Key::Hash(hash) => {
            contract_hash = ContractHash::from(hash);
        }
        _ => {
            runtime::revert(ApiError::User(1));
        }
    }
    // get the deposit purse from the contract
    let deposit_purse: URef =
        runtime::call_contract(contract_hash, "get_deposit_purse", runtime_args! {});
    //put the deposit purse in named keys
    runtime::put_key("deposit_purse", deposit_purse.into());
    system::transfer_from_purse_to_purse(account::get_main_purse(), deposit_purse, amount, None)
        .unwrap_or_revert();
    let runtime_args = runtime_args! {
        "recipient" => target,
        "amount" => amount
    };
    runtime::call_contract::<()>(contract_hash, "transfer_tokens", runtime_args);
}
