use elrond_wasm::{elrond_codec::TopEncode, formatter::SCDisplay};

use super::equippable_nft_attributes::EquippableNftAttributes;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, Debug)]
pub struct EquippableAttributesToRender<M: ManagedTypeApi> {
    pub attributes: EquippableNftAttributes<M>,
    pub name: ManagedBuffer<M>,
}

impl<M: ManagedTypeApi> SCDisplay for EquippableAttributesToRender<M> {
    fn fmt<F: elrond_wasm::formatter::FormatByteReceiver>(&self, f: &mut F) {
        let mut attributes = ManagedBuffer::<F::Api>::new_from_bytes(b"");
        let _ = self.attributes.top_encode(&mut attributes);

        let mut name = ManagedBuffer::<F::Api>::new_from_bytes(b"");
        let _ = self.name.top_encode(&mut name);

        f.append_managed_buffer(&attributes);
        f.append_bytes(b"@");
        f.append_managed_buffer(&name);
    }
}
