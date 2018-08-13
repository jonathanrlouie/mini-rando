use rand;
use bitflags;

bitflags! {
    pub struct ItemFlags: u64 {
        const ITEM_1       = 0x01;
        const ITEM_2       = 0x01 << 1;
        const ITEM_3       = 0x01 << 2;
    }
}

pub fn contains_item(items: ItemFlags, item: ItemFlags) -> bool {
	!(items & item).is_empty()
}
