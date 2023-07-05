#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::vec::Vec;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, URef, U512};
use my_package::{
    constants::{ENTRYPOINT_GET_DEPOSIT_PURSE, ENTRYPOINT_TRADE_NFT},
    Meta, TokenId,
};

#[no_mangle]
pub extern "C" fn call() {
    let amount: U512 = runtime::get_named_arg("amount");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let token_metas = runtime::get_named_arg::<Vec<Meta>>("token_metas");

    let contract_hash = {
        let contract_hashs: Key = runtime::get_named_arg("contract_hash");
        match contract_hashs {
            Key::Hash(hash) => ContractHash::from(hash),
            _ => {
                runtime::revert(ApiError::User(1));
            }
        }
    };
    // get the deposit purse from the contract
    let deposit_purse: URef = runtime::call_contract(
        contract_hash,
        ENTRYPOINT_GET_DEPOSIT_PURSE,
        runtime_args! {},
    );

    // deposit amount to the deposit purse
    system::transfer_from_purse_to_purse(account::get_main_purse(), deposit_purse, amount, None)
        .unwrap_or_revert();
    let runtime_args = runtime_args! {
        "amount" => amount,
        "token_ids" => token_ids,
        "token_metas" => token_metas
    };
    runtime::call_contract::<()>(contract_hash, ENTRYPOINT_TRADE_NFT, runtime_args);
}
