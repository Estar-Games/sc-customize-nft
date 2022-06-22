use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::{elrond_codec::TopDecode, types::ManagedBuffer};
use elrond_wasm_debug::{managed_buffer, DebugApi};

#[test]
fn decode_equippable_nft() {
    DebugApi::dummy();

    let input_data = b"Hat:Pirate Hat";
    let input_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(input_data);

    let expected_output = EquippableNftAttributes::new(&[(
        &ManagedBuffer::new_from_bytes(b"hat"),
        Item::<DebugApi> {
            name: managed_buffer!(b"Pirate Hat"),
        },
    )]);

    let actual_output = EquippableNftAttributes::top_decode(input_buffer).unwrap();

    assert_eq!(expected_output, actual_output);
}

#[test]
fn decode_empty_equippable_nft() {
    DebugApi::dummy();

    let attributes_buffer = ManagedBuffer::<DebugApi>::new_from_bytes(b"");
    let actual_output = EquippableNftAttributes::<DebugApi>::top_decode(attributes_buffer).unwrap();

    assert_eq!(EquippableNftAttributes::empty(), actual_output);
}
