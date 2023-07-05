#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
use alloc::{borrow::ToOwned, string::ToString, vec, vec::Vec};

use my_package::{
    constants::{
        CONTRACT_PURSE, ENTRYPOINT_GET_DEPOSIT_PURSE, ENTRYPOINT_TRADE_NFT, NFT_CONTRACT_HASH,
    },
    Meta, TokenId,
};

use casper_contract::{
    contract_api::{runtime, storage, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, ApiError, CLType, CLValue, ContractHash, EntryPoint,
    EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, URef, U512,
};

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let caller_purse_opt = runtime::get_key(&runtime::get_caller().to_string());
    if caller_purse_opt.is_none() {
        //create a new purse for caller and put it into the contract named_keys
        let purse = system::create_purse();
        let purse_add = purse.into_add();

        runtime::put_key(&runtime::get_caller().to_string(), purse.into());
        runtime::ret(CLValue::from_t(purse_add).unwrap_or_revert());
    } else {
        let caller_purse = caller_purse_opt.unwrap_or_revert().into_uref().unwrap();
        let caller_purse_add = caller_purse.into_add();
        runtime::ret(CLValue::from_t(caller_purse_add).unwrap_or_revert());
    }
}

#[no_mangle]
pub extern "C" fn trade_NFT() {
    let amount: U512 = runtime::get_named_arg("amount");
    // let token_ids = runtime::get_named_arg::<Vec<TokenId>>("token_ids");
    // let token_metas = runtime::get_named_arg::<Vec<Meta>>("token_metas");
    let token_ids: Vec<TokenId> = runtime::get_named_arg("token_ids");
    let token_metas = runtime::get_named_arg::<Vec<Meta>>("token_metas");

    // set a range for the amount check

    let caller_purse_opt = runtime::get_key(&runtime::get_caller().to_string());
    if caller_purse_opt.is_none() {
        runtime::revert(ApiError::User(5));
    }

    // get caller purse which pays for trading NFT
    let caller_purse: URef = caller_purse_opt.unwrap_or_revert().into_uref().unwrap();

    // get contract purse which receives payment for trading NFT
    let contract_purse: URef = runtime::get_key(CONTRACT_PURSE)
        .unwrap()
        .into_uref()
        .unwrap();

    // get nft contract hash
    let nft_contract_hash = runtime::get_key(NFT_CONTRACT_HASH)
        .unwrap()
        .into_hash()
        .map(ContractHash::new)
        .unwrap();

    // call mint/transfer NFT
    // let recipient = runtime::get_named_arg::<Key>("recipient");

    runtime::call_contract::<()>(
        nft_contract_hash,
        "mint",
        runtime_args! {
        "recipient" => Key::from(runtime::get_caller()),
        "token_ids" => token_ids,
        "token_metas" => token_metas},
    );

    // transfer the payment to this contract
    system::transfer_from_purse_to_purse(caller_purse, contract_purse, amount, None).unwrap();
}

#[no_mangle]
pub extern "C" fn call() {
    // create contract purse
    let named_keys = {
        let mut named_keys = NamedKeys::new();

        // create contract purse and store it into named_keys
        let contract_purse = system::create_purse();
        named_keys.insert(CONTRACT_PURSE.to_owned(), contract_purse.into());

        // store NFT_CONTRACT_HASH into named_keys
        let nft_contract_hash_key: Key = runtime::get_named_arg("nft_contract");
        let nft_contract_hash: ContractHash = nft_contract_hash_key
            .into_hash()
            .map(ContractHash::new)
            .unwrap();
        named_keys.insert(NFT_CONTRACT_HASH.to_owned(), nft_contract_hash.into());
        named_keys
    };

    let mut entry_points = EntryPoints::new();

    let entry_point_1 = EntryPoint::new(
        ENTRYPOINT_GET_DEPOSIT_PURSE,
        vec![
            Parameter::new("user_purse", CLType::URef),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let entry_point_2 = EntryPoint::new(
        ENTRYPOINT_TRADE_NFT,
        vec![
            Parameter::new("recipient", CLType::Key),
            Parameter::new("amount", CLType::U512),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(entry_point_1);
    entry_points.add_entry_point(entry_point_2);

    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("trade_NFT_package".to_string()),
        None,
    );

    // store contract hash and package hash inside the named_keys of installer
    runtime::put_key("trade_NFT_contract", contract_hash.into());
}
