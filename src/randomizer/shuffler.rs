use rand::{Rng, StdRng};
use super::{
    item::LabelledItem,
    location::Location
};

pub struct Shuffled(pub Vec<Location>, pub Vec<LabelledItem>, pub Vec<LabelledItem>);

pub fn shuffle_world(
    rng: &mut StdRng,
    mut locations: Vec<Location>,
    mut prog_items: Vec<LabelledItem>,
    mut junk_items: Vec<LabelledItem>
) -> Shuffled {
    rng.shuffle(&mut prog_items);
    rng.shuffle(&mut locations);
    rng.shuffle(&mut junk_items);
    Shuffled(locations, prog_items, junk_items)
}