#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::ToString, vec};

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, contracts::NamedKeys, ApiError, CLType, CLValue, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, URef, U512,
};

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let caller_purse_opt = runtime::get_key(&runtime::get_caller().to_string());
    if caller_purse_opt.is_none() {
        //create a new purse for caller and put it into the deposit dict
        let purse = system::create_purse();
        let purse_add = purse.into_add();
        // storage::dictionary_put(deposit_dict, caller.to_string().as_str(), purse);
        runtime::put_key(&runtime::get_caller().to_string(), purse.into());
        runtime::ret(CLValue::from_t(purse_add).unwrap_or_revert());
    } else {
        let caller_purse = caller_purse_opt.unwrap_or_revert().into_uref().unwrap();
        let caller_purse_add = caller_purse.into_add();
        runtime::ret(CLValue::from_t(caller_purse_add).unwrap_or_revert());
    }
}

#[no_mangle]
pub extern "C" fn transfer_tokens() {
    let target_key: Key = runtime::get_named_arg("recipient");
    let target: AccountHash;
    match target_key {
        Key::Account(target_account) => target = target_account,
        _ => {
            runtime::revert(ApiError::User(4));
        }
    }
    let amount: U512 = runtime::get_named_arg("amount");

    let caller_purse_opt = runtime::get_key(&runtime::get_caller().to_string());
    if caller_purse_opt.is_none() {
        runtime::revert(ApiError::User(5));
    }
    let caller_purse: URef = caller_purse_opt.unwrap_or_revert().into_uref().unwrap();
    system::transfer_from_purse_to_account(caller_purse, target, amount, None).unwrap();
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    let entry_point_2 = EntryPoint::new(
        "get_deposit_purse",
        vec![
            Parameter::new("user_purse", CLType::URef),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let entry_point_3 = EntryPoint::new(
        "transfer_tokens",
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(entry_point_2);
    entry_points.add_entry_point(entry_point_3);

    // access - contract
    let (contract_package_hash, _access_uref) = storage::create_contract_package_at_hash();

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, NamedKeys::default());

    runtime::put_key("transfer_cspr_contract", contract_hash.into());
    runtime::put_key("transfer_cspr_package", contract_package_hash.into());
}
