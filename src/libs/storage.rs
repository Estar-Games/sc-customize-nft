use crate::structs::penguin_attributes::PenguinAttributes;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("penguins_identifier")]
    fn penguins_identifier(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("ipfsGateway")]
    fn ipfs_gateway(&self) -> SingleValueMapper<ManagedBuffer<Self::Api>>;

    #[storage_mapper("items_types")]
    fn slot_of(&self, token: &TokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("penguin_cid_by_attributes")]
    fn thumbnail_cid_of(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
    ) -> SingleValueMapper<ManagedBuffer>;

    // STORAGE MODIFIERS

    #[endpoint]
    #[only_owner]
    fn set_thumbnail_cid(
        &self,
        attributes: &PenguinAttributes<Self::Api>,
        cid: ManagedBuffer<Self::Api>,
    ) {
        self.thumbnail_cid_of(attributes).set(cid);
    }

    fn has_slot(&self, _token: &TokenIdentifier) -> bool {
        panic!("Not implemented");
    }
}
