use crate::{args_set_cid_of, testing_utils};
use customize_nft::libs::storage::StorageModule;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm_debug::DebugApi;

use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;

#[test]
fn build_url_with_no_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let equippable_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
                &ManagedBuffer::new_from_bytes(b"hat"),
                Item::<DebugApi> {
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                },
            )]);

            let _ = sc.get_uri_of(&equippable_attributes);
        })
        .assert_user_error("There is no CID associated to the attributes Hat:item name.");
}

#[test]
fn build_url_with_associated_cid() {
    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |sc| {
            let penguin_attributes = EquippableNftAttributes::<DebugApi>::new(&[(
                &ManagedBuffer::new_from_bytes(b"hat"),
                Item::<DebugApi> {
                    name: ManagedBuffer::new_from_bytes(b"item name"),
                },
            )]);

            sc.set_cid_of(args_set_cid_of!(
                penguin_attributes.clone(),
                ManagedBuffer::new_from_bytes(b"this is a CID")
            ));

            sc.ipfs_gateway()
                .set(ManagedBuffer::new_from_bytes(b"https://ipfs.io/ipfs/"));

            let url = sc.get_uri_of(&penguin_attributes);

            assert_eq!(
                url,
                ManagedBuffer::from(b"https://ipfs.io/ipfs/this is a CID")
            )
        })
        .assert_ok();
}
