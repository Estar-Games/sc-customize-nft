elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use elrond_wasm::types::ManagedVec;

use crate::structs::equippable_nft_attributes::EquippableNftAttributes;

#[elrond_wasm::module]
pub trait MintEquippableModule: super::storage::StorageModule {
    /// Burn old equipable, and mint a new one.
    fn update_equippable(
        &self,
        equippable_nonce: u64,
        attributes: &EquippableNftAttributes<Self::Api>,
    ) -> u64 {
        let equippable_token_id = self.equippable_token_id().get();
        let caller = self.blockchain().get_caller();

        let equippable_name = self
            .blockchain()
            .get_esdt_token_data(
                &self.blockchain().get_sc_address(),
                &equippable_token_id,
                equippable_nonce,
            )
            .name;

        // mint
        let token_nonce = self.mint_equippable(attributes, &equippable_name);

        // burn the old one
        self.send()
            .esdt_local_burn(&equippable_token_id, equippable_nonce, &BigUint::from(1u32));

        // send the new one
        self.send().direct_esdt(
            &caller,
            &equippable_token_id,
            token_nonce,
            &BigUint::from(1u32),
            &[],
        );

        return token_nonce;
    }

    fn mint_equippable(
        &self,
        attributes: &EquippableNftAttributes<Self::Api>,
        name: &ManagedBuffer,
    ) -> u64 {
        let mut uris = ManagedVec::new();
        let thumbnail = self.get_uri_of(&attributes);
        uris.push(thumbnail);

        let token_nonce = self
            .send()
            .esdt_nft_create::<EquippableNftAttributes<Self::Api>>(
                &self.equippable_token_id().get(),
                &BigUint::from(1u32),
                &name,
                &BigUint::zero(),
                &ManagedBuffer::new(),
                &attributes,
                &uris,
            );

        return token_nonce;
    }
}
