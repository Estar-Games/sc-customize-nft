use crate::testing_utils;
use customize_nft::{libs::storage::StorageModule, structs::item::Item, Equip};
use elrond_wasm::{contract_base::ContractBase, types::EgldOrEsdtTokenIdentifier};
use elrond_wasm_debug::{managed_buffer, managed_token_id, rust_biguint};

#[test]
fn works_if_is_the_owner() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 654;

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.owner_address,
        &TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(1u64),
        &Option::Some({}),
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_esdt_transfer(
            &setup.owner_address,
            &setup.cf_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1),
            |sc| {
                let payment = sc.call_value().single_esdt();

                let _ = sc.fill(
                    EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier),
                    payment.token_nonce,
                    payment.amount,
                );

                let (item_id, item_nonce) = sc
                    .token_of(&Item {
                        name: managed_buffer!(TOKEN_ID),
                    })
                    .get();

                assert_eq!(item_id, managed_token_id!(TOKEN_ID));
                assert_eq!(item_nonce, TOKEN_NONCE);
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_not_the_owner() {
    const TOKEN_ID: &[u8] = b"ITEM-a1a1a1";
    const TOKEN_NONCE: u64 = 654;

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        &setup.first_user_address,
        &TOKEN_ID,
        TOKEN_NONCE,
        &rust_biguint!(1u64),
        &Option::Some({}),
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_esdt_transfer(
            &setup.first_user_address,
            &setup.cf_wrapper,
            TOKEN_ID,
            TOKEN_NONCE,
            &rust_biguint!(1),
            |sc| {
                let payment = sc.call_value().single_esdt();

                let _ = sc.fill(
                    EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier),
                    payment.token_nonce,
                    payment.amount,
                );
            },
        )
        .assert_user_error("Only the owner can call this method.");
}
