extern crate amethyst;
extern crate tiled;
extern crate rand;

mod randomizer;

use randomizer::filler::{fill_locations, fast_filler};
use randomizer::location::{Location};
use randomizer::item::{Item, LabelledItem};

const FRAME_LIMIT: u32 = 60;

fn main() {
    let locations: Vec<Location> = vec![
        Location::Location0,
        Location::Location1,
        Location::Location2
    ];

    let prog_items: Vec<LabelledItem> = vec![
        LabelledItem::Progression(Item::Item0),
        LabelledItem::Progression(Item::Item1),
        LabelledItem::Progression(Item::Item2)
    ];

    let filled_locations = fast_filler(prog_items, locations);

    println!("{:?}, {:?}, {:?}", filled_locations[0], filled_locations[1], filled_locations[2]);
    //fill_locations(locations, prog_items);
}
