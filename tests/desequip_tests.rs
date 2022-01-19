use elrond_wasm::types::{ManagedBuffer, ManagedVarArgs, SCResult};
use elrond_wasm_debug::tx_mock::TxInputESDT;
use elrond_wasm_debug::{managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};
use equip_penguin::*;

mod utils;

const PENGUIN_TOKEN_ID: &[u8] = utils::utils::PENGUIN_TOKEN_ID;
const HAT_TOKEN_ID: &[u8] = utils::utils::HAT_TOKEN_ID;
const INIT_NONCE: u64 = 65535;

#[test]
fn test_desequip() {
    let mut setup = utils::utils::setup(equip_penguin::contract_obj);

    let b_wrapper = &mut setup.blockchain_wrapper;

    let mut transfers = Vec::new();
    transfers.push(TxInputESDT {
        token_identifier: PENGUIN_TOKEN_ID.to_vec(),
        nonce: INIT_NONCE,
        value: rust_biguint!(1),
    });

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(0),
        &ItemAttributes {},
    );

    b_wrapper.set_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        INIT_NONCE,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from_esdt_bytes(HAT_TOKEN_ID),
                INIT_NONCE,
            ),
        },
    );

    b_wrapper.execute_esdt_multi_transfer(
        &setup.first_user_address,
        &setup.cf_wrapper,
        &transfers,
        |sc| {
            let mut managed_slots = ManagedVarArgs::<DebugApi, ItemSlot>::new();
            managed_slots.push(ItemSlot::Hat);

            let result = sc.desequip(
                &TokenIdentifier::<DebugApi>::from_esdt_bytes(PENGUIN_TOKEN_ID),
                INIT_NONCE,
                managed_slots,
            );

            assert_eq!(result, SCResult::Ok(1u64));

            StateChange::Commit
        },
    );

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        PENGUIN_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        &PenguinAttributes {
            hat: (
                TokenIdentifier::<DebugApi>::from(ManagedBuffer::new()),
                0u64,
            ),
        },
    );

    b_wrapper.check_nft_balance(
        &setup.first_user_address,
        HAT_TOKEN_ID,
        1u64,
        &rust_biguint!(1),
        &ItemAttributes {},
    )
}
