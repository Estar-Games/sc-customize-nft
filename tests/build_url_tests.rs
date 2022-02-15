use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::TokenIdentifier;
use elrond_wasm_debug::managed_token_id;
use elrond_wasm_debug::rust_biguint;
use elrond_wasm_debug::testing_framework::StateChange;
use elrond_wasm_debug::DebugApi;
use equip_penguin::item_slot::ItemSlot;
use equip_penguin::Equip;
use equip_penguin::{
    item::Item, item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes,
};

mod utils;

#[test]
fn build_url_with_one_item() {
    // utils::execute_for_all_slot(|mut slot| {
    let slot = &ItemSlot::Hat;

    let mut setup = utils::setup(equip_penguin::contract_obj);

    const ITEM_IDENTIFIER: &[u8] = b"ITEM-a1a1a1";
    const ITEM_TYPE: &[u8] = b"my-item-id";
    const NONCE: u64 = 6000;

    // create item
    setup.register_item(slot.clone(), ITEM_IDENTIFIER);

    setup.blockchain_wrapper.set_nft_balance(
        setup.cf_wrapper.address_ref(),
        ITEM_IDENTIFIER,
        NONCE,
        &rust_biguint!(1),
        &ItemAttributes::<DebugApi> {
            item_id: ManagedBuffer::<DebugApi>::new_from_bytes(ITEM_TYPE),
        },
    );

    let b_wrapper = &mut setup.blockchain_wrapper;

    let _ = b_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            // instantiate penguin with item
            let penguin_attributes = PenguinAttributes::<DebugApi> {
                hat: Some(Item::<DebugApi> {
                    token: TokenIdentifier::<DebugApi>::from_esdt_bytes(ITEM_IDENTIFIER),
                    nonce: NONCE,
                }),
                ..PenguinAttributes::empty()
            };

            let actual = sc.build_url(&penguin_attributes);

            assert!(actual.is_ok());

            let mut expected = ManagedBuffer::new();
            expected.append(&sc.uri().get());
            expected.append(&ManagedBuffer::new_from_bytes(slot.to_bytes::<DebugApi>())); // slot to string eg. skin
            expected.append_bytes(b"_");
            expected.append(&ManagedBuffer::new_from_bytes(ITEM_TYPE)); // slot value eg. albino
            expected.append_bytes(b"/image.png");

            assert_eq!(actual.unwrap(), expected);
        })
        .assert_ok();
}