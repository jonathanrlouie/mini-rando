extern crate amethyst;
extern crate tiled;
#[macro_use]
extern crate bitflags;
extern crate rand;

mod randomizer;

use randomizer::item::{ItemSet, contains_item};

const FRAME_LIMIT: u32 = 60;

fn main() -> () {
	let inventory = ItemSet::empty();
	let item = ItemSet::ITEM_1;
	let new_inventory = inventory | item;
    println!("{}", contains_item(new_inventory, item));
}