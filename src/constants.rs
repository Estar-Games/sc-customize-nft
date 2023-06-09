pub const ENQUEUE_PRICE: u64 = 1_000_000_000_000_000; // 0.001 EGLD
pub const UNEQUIPPED_ITEM_NAME: &[u8] = b"unequipped";

pub const ERR_CANNOT_REGISTER_EQUIPPABLE_AS_ITEM: &str =
    "You cannot register an equippable NFT as an item.";
pub const ERR_NEED_EQUIPPABLE: &str = "You must send the equippable NFT to equip.";
pub const ERR_NEED_ONE_ITEM_OR_UNEQUIP_SLOT: &str =
    "You must either send an item to equip OR set a slot to unequip.";
pub const ERR_FIRST_PAYMENT_IS_EQUIPPABLE: &str =
    "The first token sent must be the equippable NFT.";
pub const ERR_MORE_THAN_ONE_EQUIPPABLE_RECEIVED: &str =
    "Sending more than one equippable NFT is not supported.";
pub const ERR_MORE_THAN_ONE_ITEM_RECEIVED: &str = "Sending more than one item is not supported.";
pub const ERR_CANNOT_EQUIP_EQUIPPABLE: &str =
    "cannot equip an equippable NFT over another equippable NFT.";
pub const ERR_CREATE_ROLE_NOT_SET_FOR_EQUIPPABLE: &str =
    "This smart contract lacks the create role in the collection of equipable NFTs.";
pub const ERR_BURN_ROLE_NOT_SET_FOR_EQUIPPABLE: &str =
    "This smart contract lacks the burn role in the collection of equipable NFTs.";
pub const ERR_CANNOT_UNEQUIP_EMPTY_SLOT: &str = "cannot unequip an empty slot";
pub const ERR_ITEM_TO_UNEQUIP_HAS_NO_SLOT: &str =
    "Item to unequip has no slot. Please, contact an admin.";
pub const ERR_CANNOT_ENQUEUE_IMAGE_BECAUSE_ALREADY_RENDERED: &str =
    "We can't enqueue this image, because it has already been rendered";
pub const ERR_RENDER_ALREADY_IN_QUEUE: &str = "This image is already in the queue";
pub const ERR_IMAGE_NOT_IN_QUEUE: &str = "This image is not in the queue";
pub const ERR_PAY_0001_EGLD: &str = "You must pay 0.001 EGLD to call this endpoint.";
pub const ERR_CANNOT_OVERRIDE_URI_OF_ATTRIBUTE: &str = "Another URI has been set previously.";
pub const ERR_CANNOT_OVERRIDE_REGISTERED_ITEM: &str = "Item is already registered.";
pub const ERR_CANNOT_FILL_UNREGISTERED_ITEM: &str =
    "An item must be registered before calling the fill endpoint.";
pub const ERR_IMAGE_NOT_IN_RENDER_QUEUE: &str =
    "cannot set the uri because the attributes are not in the render queue";
