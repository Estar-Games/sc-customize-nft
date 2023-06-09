use customize_nft::constants::{
    ERR_CANNOT_OVERRIDE_REGISTERED_ITEM, ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM,
    UNEQUIPPED_ITEM_NAME,
};
use customize_nft::libs::storage::StorageModule;
use customize_nft::structs::equippable_attributes::{
    ERR_NAME_CANNOT_BE_UNEQUIPPED, ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS,
    ERR_SLOT_CONTAINS_UNSUPPORTED_CHARACTERS,
};
use customize_nft::structs::item::Item;
use customize_nft::structs::token::Token;
use customize_nft::*;
use elrond_wasm::elrond_codec::multi_types::MultiValue4;
use elrond_wasm::types::{MultiValueEncoded, TokenIdentifier};
use elrond_wasm_debug::tx_mock::TxResult;
use elrond_wasm_debug::{managed_buffer, managed_token_id};
use elrond_wasm_debug::{rust_biguint, DebugApi};

use crate::testing_utils::{self, TestItemAttributes};

#[test]
fn test_register_item() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 42;
    const ITEM_NAME: &[u8] = b"Pirate Hat";

    DebugApi::dummy();

    setup.register_and_fill_item(
        slot,
        ITEM_NAME,
        TOKEN_ID,
        TOKEN_NONCE,
        &TestItemAttributes {},
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let result = sc.get_item(&Token::new(
                TokenIdentifier::from_esdt_bytes(TOKEN_ID),
                TOKEN_NONCE,
            ));

            assert_eq!(result.is_some(), true);
            assert_eq!(
                result.unwrap(),
                Item {
                    slot: managed_buffer!(slot),
                    name: managed_buffer!(ITEM_NAME)
                }
            );
        })
        .assert_ok();
}

/// Ce test vérifie que si on associe 2 items au même slot, tout fonctionne bien
#[test]
fn register_another_item_on_slot() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const FIRST_TOKEN_ID: &[u8] = b"a";
    const FIRST_TOKEN_NONCE: u64 = 42;
    const FIRST_ITEM_NAME: &[u8] = b"first item";

    const SECOND_TOKEN_ID: &[u8] = b"A";
    const SECOND_TOKEN_NONCE: u64 = 43;
    const SECOND_ITEM_NAME: &[u8] = b"second item";

    const COMMON_SLOT: &[u8] = b"slot";

    DebugApi::dummy();
    setup.register_and_fill_item(
        COMMON_SLOT,
        FIRST_ITEM_NAME,
        FIRST_TOKEN_ID,
        FIRST_TOKEN_NONCE,
        &TestItemAttributes {},
    );
    setup.register_and_fill_item(
        COMMON_SLOT,
        SECOND_ITEM_NAME,
        SECOND_TOKEN_ID,
        SECOND_TOKEN_NONCE,
        &TestItemAttributes {},
    );

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            assert_eq!(
                sc.get_item(&Token::new(
                    managed_token_id!(FIRST_TOKEN_ID),
                    FIRST_TOKEN_NONCE
                ))
                .unwrap(),
                Item {
                    slot: managed_buffer!(COMMON_SLOT),
                    name: managed_buffer!(FIRST_ITEM_NAME)
                }
            );

            assert_eq!(
                sc.get_item(&Token::new(
                    managed_token_id!(SECOND_TOKEN_ID),
                    SECOND_TOKEN_NONCE
                ))
                .unwrap(),
                Item {
                    slot: managed_buffer!(COMMON_SLOT),
                    name: managed_buffer!(SECOND_ITEM_NAME)
                }
            );
        })
        .assert_ok();
}

#[test]
fn panic_if_override() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    const TOKEN_ID: &[u8] = b"HAT-a1a1a1";
    const TOKEN_NONCE: u64 = 1;

    let first_slot = b"hat";
    let first_slot_item_name = b"pirate hat";

    let second_slot = b"clothes";
    let second_slot_item_name = b"Golden Chain";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    managed_buffer!(first_slot),
                    managed_buffer!(first_slot_item_name),
                    managed_token_id!(TOKEN_ID),
                    TOKEN_NONCE,
                )));

                sc.register_item(items);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    managed_buffer!(second_slot),
                    managed_buffer!(second_slot_item_name),
                    managed_token_id!(TOKEN_ID),
                    TOKEN_NONCE,
                )));

                sc.register_item(items);
            },
        )
        .assert_user_error(ERR_CANNOT_OVERRIDE_REGISTERED_ITEM);
}

#[test]
fn panic_if_register_equippable() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    let slot = b"hat";

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut items = MultiValueEncoded::new();
                items.push(MultiValue4::from((
                    managed_buffer!(slot),
                    managed_buffer!(b"My Equippable"),
                    managed_token_id!(testing_utils::EQUIPPABLE_TOKEN_ID),
                    1,
                )));

                sc.register_item(items);
            },
        )
        .assert_user_error(ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM);
}

#[test]
fn panic_if_not_the_owner() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.first_user_address,
            &setup.cf_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.call_register_item();
            },
        )
        .assert_user_error("Endpoint can only be called by owner");
}

#[test]
fn panic_if_name_contains_two_dots() {
    call_register_item(b"hat", b"My:Hat", b"HAT-a1a1a1")
        .assert_user_error(std::str::from_utf8(ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_name_contains_semicolon() {
    call_register_item(b"hat", b"My;Hat", b"HAT-a1a1a1")
        .assert_user_error(std::str::from_utf8(ERR_NAME_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_name_equals_unequipped() {
    call_register_item(b"hat", UNEQUIPPED_ITEM_NAME, b"HAT-a1a1a1")
        .assert_user_error(std::str::from_utf8(ERR_NAME_CANNOT_BE_UNEQUIPPED).unwrap());
}

#[test]
fn panic_if_slot_contains_semicolon() {
    call_register_item(b"ha;t", b"My Hat", b"HAT-a1a1a1")
        .assert_user_error(std::str::from_utf8(ERR_SLOT_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

#[test]
fn panic_if_slot_contains_twodots() {
    call_register_item(b"ha:t", b"My Hat", b"HAT-a1a1a1")
        .assert_user_error(std::str::from_utf8(ERR_SLOT_CONTAINS_UNSUPPORTED_CHARACTERS).unwrap());
}

fn call_register_item(slot: &[u8], name: &[u8], token_id: &[u8]) -> TxResult {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    return setup.blockchain_wrapper.execute_tx(
        &setup.owner_address,
        &setup.cf_wrapper,
        &rust_biguint!(0u64),
        |sc| {
            let mut items = MultiValueEncoded::new();
            items.push(MultiValue4::from((
                managed_buffer!(slot),
                managed_buffer!(name),
                managed_token_id!(token_id),
                1,
            )));

            sc.register_item(items);
        },
    );
}
