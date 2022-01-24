use std::u8;

use elrond_wasm::types::{
    Address, EsdtLocalRole, ManagedMultiResultVec, ManagedVarArgs, MultiArg2, SCResult,
};
use elrond_wasm_debug::tx_mock::{TxContextRef, TxInputESDT};
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::penguins_attributes::PenguinAttributes;
use equip_penguin::*;

const WASM_PATH: &'static str = "sc-equip-penguin/output/equip_penguin.wasm";

pub const PENGUIN_TOKEN_ID: &[u8] = b"PENG-ae5a";
pub const HAT_TOKEN_ID: &[u8] = b"HAT-a";
pub const HAT_2_TOKEN_ID: &[u8] = b"HAT-b";

pub struct EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper:
        ContractObjWrapper<equip_penguin::ContractObj<DebugApi>, CrowdfundingObjBuilder>,
}

pub fn setup<TObjBuilder>(cf_builder: TObjBuilder) -> EquipSetup<TObjBuilder>
where
    TObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let first_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    // deploy contract
    blockchain_wrapper.execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
        let result = sc.init(managed_token_id!(PENGUIN_TOKEN_ID));
        assert_eq!(result, SCResult::Ok(()));

        StateChange::Commit
    });
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    DebugApi::dummy();

    let contract_roles = [
        EsdtLocalRole::NftCreate,
        EsdtLocalRole::NftBurn,
        EsdtLocalRole::NftAddQuantity,
        EsdtLocalRole::Mint,
        EsdtLocalRole::Burn,
    ];

    blockchain_wrapper.set_esdt_local_roles(
        cf_wrapper.address_ref(),
        PENGUIN_TOKEN_ID,
        &contract_roles,
    );

    blockchain_wrapper.set_esdt_local_roles(
        cf_wrapper.address_ref(),
        HAT_TOKEN_ID,
        &contract_roles,
    );

    let equip_setup = EquipSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    };

    // register items
    // register_item(&mut equip_setup, ItemSlot::Hat, HAT_TOKEN_ID);

    return equip_setup;
}

pub fn register_item<EquipObjBuilder>(
    setup: &mut EquipSetup<EquipObjBuilder>,
    item_type: ItemSlot,
    item_id: &[u8],
) where
    EquipObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let mut managed_items_ids =
                ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
            managed_items_ids.push(managed_token_id!(item_id));

            let result = sc.register_item(item_type, managed_items_ids);

            if let SCResult::Err(err) = result {
                panic!(
                    "register_item {:?} failed: {:?}",
                    std::str::from_utf8(&item_id).unwrap(),
                    std::str::from_utf8(&err.as_bytes()).unwrap(),
                );
            }

            assert_eq!(result, SCResult::Ok(()));

            StateChange::Commit
        },
    );
}

pub fn verbose_log_if_error<T>(result: &SCResult<T>, message: String) {
    if let SCResult::Err(err) = &*result {
        panic!(
            "{} | failed {:?}",
            message,
            std::str::from_utf8(&err.as_bytes()).unwrap(),
        );
    }
}

pub fn create_esdt_transfers(tokens: &[(&[u8], u64)]) -> Vec<TxInputESDT> {
    let mut transfers = Vec::new();

    for (token_id, nonce) in tokens {
        transfers.push(TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce: nonce.clone(),
            value: rust_biguint!(1u64),
        })
    }

    return transfers;
}

pub fn create_managed_items_to_equip(
    tokens: &[(&[u8], u64)],
) -> ManagedMultiResultVec<
    TxContextRef,
    MultiArg2<elrond_wasm::types::TokenIdentifier<TxContextRef>, u64>,
> {
    let mut managed_items_to_equip =
        ManagedVarArgs::<DebugApi, MultiArg2<TokenIdentifier<DebugApi>, u64>>::new();

    for (token_id, nonce) in tokens {
        managed_items_to_equip.push(MultiArg2((
            TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.clone()),
            nonce.clone(),
        )));
    }

    return managed_items_to_equip;
}

pub fn set_all_permissions_on_token<EquipObjBuilder>(
    setup: &mut EquipSetup<EquipObjBuilder>,
    token_id: &[u8],
) where
    EquipObjBuilder: 'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
{
    let contract_roles = [
        EsdtLocalRole::NftCreate,
        EsdtLocalRole::NftBurn,
        EsdtLocalRole::NftAddQuantity,
        EsdtLocalRole::Mint,
        EsdtLocalRole::Burn,
    ];
    setup.blockchain_wrapper.set_esdt_local_roles(
        setup.cf_wrapper.address_ref(),
        token_id,
        &contract_roles,
    );
}

pub fn give_one_penguin_with_hat(
    blockchain_wrapper: &mut BlockchainStateWrapper,
    user_address: &Address,
    penguin_nonce: u64,
    hat_nonce: u64,
) {
    blockchain_wrapper.set_nft_balance(
        &user_address,
        PENGUIN_TOKEN_ID,
        penguin_nonce,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                hat_nonce,
            ),
            ..PenguinAttributes::empty()
        },
    );
}

pub fn execute_for_all_slot(execute: fn(&ItemSlot) -> ()) {
    // execute(&ItemSlot::Hat);
    for slot in ItemSlot::VALUES.iter() {
        execute(slot);
    }
}
