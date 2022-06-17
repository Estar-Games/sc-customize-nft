use std::u8;

use customize_nft::structs::equippable_nft_attributes::EquippableNftAttributes;
use customize_nft::structs::item::Item;
use customize_nft::structs::item_attributes::ItemAttributes;
use customize_nft::*;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::types::{
    Address, BigUint, EsdtLocalRole, EsdtTokenPayment, EsdtTokenType, ManagedBuffer, ManagedVec,
    MultiValueEncoded, TokenIdentifier,
};
use elrond_wasm_debug::tx_mock::{TxInputESDT, TxResult};
use elrond_wasm_debug::{managed_buffer, managed_token_id, testing_framework::*};
use elrond_wasm_debug::{rust_biguint, DebugApi};

pub const WASM_PATH: &'static str = "sc-customize-nft/output/customize_nft.wasm";

pub const EQUIPPABLE_TOKEN_ID: &[u8] = b"PENG-ae5a";

pub const HAT_TOKEN_ID: &[u8] = b"HAT-a";

pub const INIT_NONCE: u64 = 65535u64;

pub struct EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub cf_wrapper:
        ContractObjWrapper<customize_nft::ContractObj<DebugApi>, CrowdfundingObjBuilder>,
}

impl<CrowdfundingObjBuilder> EquipSetup<CrowdfundingObjBuilder>
where
    CrowdfundingObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
{
    pub fn register_item(
        &mut self,
        slot: &[u8],
        item_id: &[u8],
        attributes: &ItemAttributes<DebugApi>,
    ) -> u64 {
        return self.register_item_all_properties(
            slot,
            item_id,
            attributes,
            0u64,
            Option::None,
            Option::None,
            Option::None,
            &[],
        );
    }

    pub fn register_item_all_properties(
        &mut self,
        slot: &[u8],
        item_id: &[u8],
        attributes: &ItemAttributes<DebugApi>,
        royalties: u64,
        creator: Option<&Address>,
        name: Option<&[u8]>,
        hash: Option<&[u8]>,
        uri: &[Vec<u8>],
    ) -> u64 {
        self.blockchain_wrapper.set_nft_balance_all_properties(
            &self.cf_wrapper.address_ref(),
            &item_id,
            INIT_NONCE,
            &rust_biguint!(2u64),
            &attributes,
            royalties,
            creator,
            name,
            hash,
            uri,
        );

        self.set_all_permissions_on_token(item_id);

        self.blockchain_wrapper
            .execute_tx(
                &self.owner_address,
                &self.cf_wrapper,
                &rust_biguint!(0u64),
                |sc| {
                    let mut managed_items_ids =
                        MultiValueEncoded::<DebugApi, TokenIdentifier<DebugApi>>::new();
                    managed_items_ids.push(managed_token_id!(item_id));

                    sc.register_item(ManagedBuffer::new_from_bytes(slot), managed_items_ids);
                },
            )
            .assert_ok();

        println!(
            "Item {:?} created and register with nonce {:x}",
            std::str::from_utf8(item_id).unwrap(),
            INIT_NONCE
        );

        return INIT_NONCE;
    }

    pub fn add_random_item_to_user(&mut self, token_id: &[u8], nonce: u64, quantity: u64) {
        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            token_id,
            nonce,
            &rust_biguint!(quantity),
            &ItemAttributes::<DebugApi>::random(),
        );
    }

    fn set_all_permissions_on_token(&mut self, token_id: &[u8]) {
        let contract_roles = [
            EsdtLocalRole::NftCreate,
            EsdtLocalRole::NftBurn,
            EsdtLocalRole::NftAddQuantity,
        ];
        self.blockchain_wrapper.set_esdt_local_roles(
            self.cf_wrapper.address_ref(),
            token_id,
            &contract_roles,
        );
    }

    pub fn create_empty_equippable(&mut self, nonce: u64) {
        DebugApi::dummy();

        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            nonce,
            &rust_biguint!(1),
            &EquippableNftAttributes::<DebugApi>::empty(),
        );
    }

    pub fn create_equippable_with_registered_item(
        &mut self,
        nonce: u64,
        item_identifier: &[u8],
        item_nonce: u64,
        slot: &[u8],
        attributes: ItemAttributes<DebugApi>,
    ) {
        self.register_item(slot, item_identifier, &attributes);

        self.blockchain_wrapper.set_nft_balance(
            &self.cf_wrapper.address_ref(),
            &item_identifier,
            item_nonce,
            &rust_biguint!(2u64),
            &attributes,
        );

        let attributes = EquippableNftAttributes::new(&[(
            &ManagedBuffer::new_from_bytes(slot),
            Item {
                token: TokenIdentifier::<DebugApi>::from_esdt_bytes(item_identifier),
                nonce: item_nonce,
                name: ManagedBuffer::new_from_bytes(b"item name"),
            },
        )]);

        self.blockchain_wrapper.set_nft_balance(
            &self.first_user_address,
            EQUIPPABLE_TOKEN_ID,
            nonce,
            &rust_biguint!(1),
            &attributes,
        );
    }

    pub fn customize(
        &mut self,
        transfers: Vec<TxInputESDT>,
        unequip_slots: &[&[u8]],
    ) -> (Option<u64>, TxResult) {
        let mut opt_sc_result: Option<u64> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let mut unequip_slots_managed =
                    MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new();

                for s in unequip_slots {
                    unequip_slots_managed.push(managed_buffer!(s));
                }

                let result =
                    sc.customize(sc.call_value().all_esdt_transfers(), unequip_slots_managed);

                opt_sc_result = Option::Some(result.clone());
            },
        );

        return (opt_sc_result, tx_result);
    }

    pub fn assert_is_burn(&mut self, token_id: &[u8], token_nonce: u64) {
        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                &self.first_user_address,
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );

        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                &self.second_user_address,
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );

        assert_eq!(
            self.blockchain_wrapper.get_esdt_balance(
                self.cf_wrapper.address_ref(),
                token_id,
                token_nonce
            ),
            rust_biguint!(0)
        );
    }

    pub fn equip(&mut self, transfers: Vec<TxInputESDT>) -> (Option<u64>, TxResult) {
        let mut opt_sc_result: Option<u64> = Option::None;

        let tx_result = self.blockchain_wrapper.execute_esdt_multi_transfer(
            &self.first_user_address,
            &self.cf_wrapper,
            &transfers,
            |sc| {
                let result = sc.customize(
                    sc.call_value().all_esdt_transfers(),
                    MultiValueEncoded::<DebugApi, ManagedBuffer<DebugApi>>::new(),
                );

                opt_sc_result = Option::Some(result);
            },
        );

        return (opt_sc_result, tx_result);
    }
}

pub fn setup<TObjBuilder>(cf_builder: TObjBuilder) -> EquipSetup<TObjBuilder>
where
    TObjBuilder: 'static + Copy + Fn() -> customize_nft::ContractObj<DebugApi>,
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
    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init(
                managed_token_id!(EQUIPPABLE_TOKEN_ID),
                managed_buffer!(b"https://ipfs.io/ipfs/"),
                managed_buffer!(b"Equippable #{number}"),
            );
        })
        .assert_ok();
    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    let mut equip_setup = EquipSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        cf_wrapper,
    };

    equip_setup.set_all_permissions_on_token(EQUIPPABLE_TOKEN_ID);

    return equip_setup;
}

pub fn create_paymens_and_esdt_transfers(
    tokens: &[(&[u8], u64, EsdtTokenType)],
) -> (
    Vec<TxInputESDT>,
    ManagedVec<DebugApi, EsdtTokenPayment<DebugApi>>,
) {
    // remove EsdtTokenType from tokens
    let mut tokens_without_type = Vec::new();
    for (token_id, nonce, _) in tokens {
        tokens_without_type.push((token_id.clone(), nonce.clone()));
    }

    return (
        create_esdt_transfers(tokens_without_type.as_slice()),
        create_payments(tokens),
    );
}

pub fn create_esdt_transfers(tokens: &[(&[u8], u64)]) -> Vec<TxInputESDT> {
    let mut transfers = Vec::new();

    for (token_id, nonce) in tokens {
        transfers.push(TxInputESDT {
            token_identifier: token_id.to_vec(),
            nonce: nonce.clone(),
            value: rust_biguint!(1u64),
        })
    }

    return transfers;
}

pub fn create_payments(
    tokens: &[(&[u8], u64, EsdtTokenType)],
) -> ManagedVec<DebugApi, EsdtTokenPayment<DebugApi>> {
    let mut payments = ManagedVec::<DebugApi, EsdtTokenPayment<DebugApi>>::new();

    for (token_id, nonce, _) in tokens {
        let payment = EsdtTokenPayment::new(
            TokenIdentifier::<DebugApi>::from_esdt_bytes(token_id.to_vec()),
            nonce.clone(),
            BigUint::from(1u64),
        );

        payments.push(payment)
    }

    return payments;
}

// TODO: register item (arg = slot)
// TODO: add quantity (arg = quantity)
