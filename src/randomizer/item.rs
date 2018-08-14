use rand;
use bitflags;

bitflags! {
    pub struct ItemSet: u64 {
        const ITEM_1 = 0x01;
        const ITEM_2 = 0x01 << 1;
        const ITEM_3 = 0x01 << 2;
    }
}

bitflags! {
	pub struct ItemLocationSet: u64 {
		const LOC_1 = 0x01;
		const LOC_2 = 0x01 << 1;
		const LOC_3 = 0x01 << 2;
	}
}

// item location entities should contain:
// - name
// - item location as ItemLocationSet
// - item as ItemSet (maybe, since we could just pass the flags to the generator)

pub fn unlocked_locations(item: ItemSet) -> Option<ItemLocationSet> {
	// map of items to the item locations they unlock
	match item {
		ItemSet::ITEM_1 => Some(ItemLocationSet::LOC_1 | ItemLocationSet::LOC_2),
		ItemSet::ITEM_2 => Some(ItemLocationSet::LOC_3),
		ItemSet::ITEM_3 => Some(ItemLocationSet::empty()),
		_ => None
	}
}

pub fn contains_item(inventory: ItemSet, item: ItemSet) -> bool {
	!(inventory & item).is_empty()
}
