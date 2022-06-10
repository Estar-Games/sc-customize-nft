use elrond_wasm::{elrond_codec::TopEncode, types::TokenIdentifier};
use elrond_wasm_debug::{managed_buffer, DebugApi};
use customize_nft::structs::{
    item::Item, item_slot::ItemSlot, penguin_attributes::PenguinAttributes,
};

#[test]
fn should_top_encode() {
    DebugApi::dummy();

    let penguin = PenguinAttributes::new(&[(
        &ItemSlot::Hat,
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 1,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected=b"Hat:Pirate Hat (HAT-a2b4e5-01);Background:unequipped;Skin:unequipped;Beak:unequipped;Weapon:unequipped;Clothes:unequipped;Eyes:unequipped";

    assert_penguin_encode_eq(penguin, expected);
}

#[test]
fn should_top_encode_with_nonce_equals_0a() {
    DebugApi::dummy();

    let penguin = PenguinAttributes::new(&[(
        &ItemSlot::Hat,
        Item::<DebugApi> {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
            nonce: 10,
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let expected=b"Hat:Pirate Hat (HAT-a2b4e5-0a);Background:unequipped;Skin:unequipped;Beak:unequipped;Weapon:unequipped;Clothes:unequipped;Eyes:unequipped";

    assert_penguin_encode_eq(penguin, expected);
}

fn assert_penguin_encode_eq(
    penguin: PenguinAttributes<elrond_wasm_debug::tx_mock::TxContextRef>,
    expected: &[u8],
) {
    let mut serialized_attributes = Vec::new();
    match penguin.top_encode(&mut serialized_attributes) {
        Ok(_) => {
            println!(
                "\n========\nActual:\n{}\n\nExpected:\n{}\n========\n",
                std::str::from_utf8(&serialized_attributes).unwrap(),
                std::str::from_utf8(expected).unwrap()
            );

            assert_eq!(
                serialized_attributes, expected,
                "top_encode should return the correct bytes"
            );
        }
        Err(err) => panic!("top_encode should not fail: {:?}", err),
    }
}

#[test]
fn encode_item_with_nonce_1() {
    DebugApi::dummy();

    let item = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 1,
        name: managed_buffer!(b"Pirate Hat"),
    };

    let expected = b"Pirate Hat (HAT-a2b4e5-01)";

    assert_item_encode_eq(item, expected);
}

#[test]
fn encode_item_with_nonce_10() {
    DebugApi::dummy();

    let item = Item::<DebugApi> {
        token: TokenIdentifier::from_esdt_bytes(b"HAT-a2b4e5"),
        nonce: 10,
        name: managed_buffer!(b"Pirate Hat"),
    };

    let expected = b"Pirate Hat (HAT-a2b4e5-0a)";

    assert_item_encode_eq(item, expected);
}

fn assert_item_encode_eq(item: Item<elrond_wasm_debug::tx_mock::TxContextRef>, expected: &[u8]) {
    let mut serialized_attributes = Vec::new();
    match item.top_encode(&mut serialized_attributes) {
        Ok(_) => {
            println!(
                "\n========\nActual:\n{}\n\nExpected:\n{}\n========\n",
                std::str::from_utf8(&serialized_attributes).unwrap(),
                std::str::from_utf8(expected).unwrap()
            );

            assert_eq!(
                serialized_attributes, expected,
                "top_encode should return the correct bytes"
            );
        }
        Err(err) => panic!("top_encode should not fail: {:?}", err),
    }
}
