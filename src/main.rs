extern crate amethyst;
extern crate tiled;
extern crate rand;

mod randomizer;

use rand::{SeedableRng, StdRng};
use randomizer::filler::{fill_locations};
use randomizer::location::{Location};
use randomizer::item::{Item, LabelledItem};

const FRAME_LIMIT: u32 = 60;

fn main() {
    let locations: Vec<Location> = vec![
        Location::Location0,
        Location::Location1,
        Location::Location2,
        Location::Location3,
        Location::Location4,
        Location::Location5
    ];

    let prog_items: Vec<LabelledItem> = vec![
        LabelledItem::Progression(Item::Item0),
        LabelledItem::Progression(Item::Item1),
        LabelledItem::Progression(Item::Item2)
    ];

    let junk_items: Vec<LabelledItem> = vec![
        LabelledItem::Junk(Item::Item3),
        LabelledItem::Junk(Item::Item3),
        LabelledItem::Junk(Item::Item3)
    ];

    let rng: StdRng = StdRng::from_seed([0u8; 32]);

    let filled_locations =
        fill_locations(
            rng,
            locations,
            prog_items,
            junk_items);

    println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
             filled_locations[0],
             filled_locations[1],
             filled_locations[2],
             filled_locations[3],
             filled_locations[4],
             filled_locations[5]
    );
}
