use elrond_wasm::{
    contract_base::ContractBase,
    types::{ManagedVarArgs, SCResult, TokenIdentifier},
};
use elrond_wasm_debug::{rust_biguint, testing_framework::StateChange, DebugApi};
use equip_penguin::{
    item::Item, item_attributes::ItemAttributes, item_slot::ItemSlot,
    penguin_attributes::PenguinAttributes, Equip,
};

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::PENGUIN_TOKEN_ID;

#[test]
fn customize_complete_flow() {
    utils::execute_for_all_slot(|slot| {
        const ITEM_TO_DESEQUIP_ID: &[u8] = b"ITEM-a1a1a1";
        const ITEM_TO_EQUIP: &[u8] = b"HAT-b2b2b2";
        const NONCE: u64 = 30;

        // 1. ARRANGE
        let mut setup = utils::setup(equip_penguin::contract_obj);

        setup.create_penguin_with_registered_item(
            NONCE,
            ITEM_TO_DESEQUIP_ID,
            NONCE,
            slot.clone(),
            ItemAttributes::random(),
        );

        setup.register_item(
            ItemSlot::Background,
            ITEM_TO_EQUIP,
            &ItemAttributes::random(),
        );

        setup.blockchain_wrapper.set_nft_balance(
            &setup.first_user_address,
            ITEM_TO_EQUIP,
            NONCE,
            &rust_biguint!(1),
            &ItemAttributes::<DebugApi>::random(),
        );

        let transfers =
            utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE), (ITEM_TO_EQUIP, NONCE)]);

        // 2. ACT
        let (sc_result, tx_result) = setup.customize(transfers, slot.clone());

        // 3. ASSERT
        tx_result.assert_ok();
        assert_eq!(sc_result, SCResult::Ok(1u64));

        // penguin&items sent burned
        setup.assert_is_burn(PENGUIN_TOKEN_ID, NONCE);
        setup.assert_is_burn(ITEM_TO_EQUIP, NONCE);

        // item desequipped received
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.first_user_address,
                ITEM_TO_DESEQUIP_ID,
                NONCE
            ),
            rust_biguint!(1)
        );

        // new desquiped penguin received
        assert_eq!(
            setup.blockchain_wrapper.get_esdt_balance(
                &setup.first_user_address,
                PENGUIN_TOKEN_ID,
                1u64
            ),
            rust_biguint!(1)
        );

        // do slot
        setup.blockchain_wrapper.check_nft_balance(
            &setup.first_user_address,
            PENGUIN_TOKEN_ID,
            1,
            &rust_biguint!(1),
            &PenguinAttributes::<DebugApi>::new(&[(
                &ItemSlot::Background,
                Item {
                    token: TokenIdentifier::from_esdt_bytes(ITEM_TO_EQUIP),
                    nonce: NONCE,
                },
            )]),
        );
    });
}

#[test]
fn customize_nothing_to_desequip_and_equip() {
    const NONCE: u64 = 30;

    // 1. ARRANGE
    let mut setup = utils::setup(equip_penguin::contract_obj);

    setup.create_penguin_empty(NONCE);

    let transfers = utils::create_esdt_transfers(&[(PENGUIN_TOKEN_ID, NONCE)]);

    // 2. ACT
    let tx_result = setup.blockchain_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let managed_slots = ManagedVarArgs::<DebugApi, ItemSlot>::new();

            let result = sc.customize(sc.call_value().all_esdt_transfers(), managed_slots);

            match result {
                SCResult::Ok(_) => StateChange::Commit,
                SCResult::Err(_) => StateChange::Revert,
            }
        },
    );

    // 3. ASSERT
    tx_result.assert_user_error(
        "You must either provide at least one penguin and one item OR provide a slot to desequip.",
    );
}