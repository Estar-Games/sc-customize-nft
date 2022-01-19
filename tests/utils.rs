#[cfg(test)]
pub mod utils {
    use elrond_wasm::types::{Address, EsdtLocalRole, ManagedVarArgs, SCResult};
    use elrond_wasm_debug::testing_framework::*;
    use elrond_wasm_debug::{rust_biguint, DebugApi};
    use equip_penguin::*;

    const WASM_PATH: &'static str = "sc-equip-penguin/output/equip_penguin.wasm";

    pub const PENGUIN_TOKEN_ID: &[u8] = b"PENG-ae5a";
    pub const HAT_TOKEN_ID: &[u8] = b"HAT-7e8f";

    // This is the nonce for the NFTs not generated from the contract but from the setup
    // Because, the contract will generate an NFT with the nonce '1', we don't want the INIT_NONCE to be '1'
    pub const INIT_NONCE: u64 = 65535;

    pub struct EquipSetup<CrowdfundingObjBuilder>
    where
        CrowdfundingObjBuilder:
            'static + Copy + Fn(DebugApi) -> equip_penguin::ContractObj<DebugApi>,
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
            let result = sc.init();
            assert_eq!(result, SCResult::Ok(()));

            StateChange::Commit
        });
        blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

        DebugApi::dummy();

        // set NFTs balance
        let none_value = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"NONE-000000");

        let nft_attributes = PenguinAttributes {
            hat: none_value.clone(),
            // background: none_value.clone(),
        };

        blockchain_wrapper.set_nft_balance(
            &first_user_address,
            PENGUIN_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &nft_attributes,
        );

        blockchain_wrapper.set_nft_balance(
            &first_user_address,
            HAT_TOKEN_ID,
            INIT_NONCE,
            &rust_biguint!(1),
            &ItemAttributes {},
        );

        blockchain_wrapper.set_esdt_local_roles(
            cf_wrapper.address_ref(),
            PENGUIN_TOKEN_ID,
            &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
        );

        blockchain_wrapper.set_esdt_local_roles(
            cf_wrapper.address_ref(),
            HAT_TOKEN_ID,
            &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn],
        );

        let mut equip_setup = EquipSetup {
            blockchain_wrapper,
            owner_address,
            first_user_address,
            second_user_address,
            cf_wrapper,
        };

        // register items
        register_item(&mut equip_setup, ItemSlot::Hat, HAT_TOKEN_ID);

        equip_setup
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
                let managed_token_id = TokenIdentifier::<DebugApi>::from_esdt_bytes(item_id);
                let mut managed_items_ids =
                    ManagedVarArgs::<DebugApi, TokenIdentifier<DebugApi>>::new();
                managed_items_ids.push(managed_token_id.clone());

                let result = sc.register_item(item_type, managed_items_ids);
                assert_eq!(result, SCResult::Ok(()));

                StateChange::Commit
            },
        );
    }
}
