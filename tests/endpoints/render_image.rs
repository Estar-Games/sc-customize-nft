use customize_nft::{
    constants::{ENQUEUE_PRICE, ERR_PAY_0001_EGLD},
    libs::equippable_uris::EquippableUrisModule,
    structs::equippable_attributes::EquippableAttributes,
};
use elrond_wasm_debug::{managed_buffer, rust_biguint, DebugApi};

use crate::testing_utils;

#[test]
fn works() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_ok();
}

#[test]
fn panic_if_dont_send_egld() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(0),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_user_error(ERR_PAY_0001_EGLD);
}

#[test]
fn panic_if_send_lesser_amount_of_egld() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE - 5),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_user_error(ERR_PAY_0001_EGLD);
}

#[test]
fn panic_if_send_greater_amount_of_egld() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(&setup.owner_address, &rust_biguint!(ENQUEUE_PRICE * 2));

    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.cf_wrapper,
            &rust_biguint!(ENQUEUE_PRICE * 2),
            |sc| {
                let attributes = EquippableAttributes::<DebugApi>::empty();
                let name = managed_buffer!(b"Equippable #512");

                sc.enqueue_image_to_render(&attributes, &name);

                assert_eq!(sc.attributes_to_render_by_name().len(), 1);
                assert_eq!(sc.attributes_to_render_by_name().contains_key(&name), true);
            },
        )
        .assert_user_error(ERR_PAY_0001_EGLD);
}
