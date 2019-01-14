use rand::{Rng, StdRng};
use super::{
    item::LabelledItem,
    location::Location
};
pub use self::shuffled::Shuffled;

pub mod shuffled {
    use super::{
        LabelledItem,
        Location
    };

    pub struct Shuffled(Vec<Location>, Vec<LabelledItem>, Vec<LabelledItem>);

    impl Shuffled {
        pub fn new(locations: Vec<Location>, prog_items: Vec<LabelledItem>, junk_items: Vec<LabelledItem>) -> Option<Self> {
            if locations.len() == prog_items.len() + junk_items.len() {
                Some(Shuffled(locations, prog_items, junk_items))
            } else {
                None
            }
        }

        pub fn get(self) -> (Vec<Location>, Vec<LabelledItem>, Vec<LabelledItem>) {
            (self.0, self.1, self.2)
        }
    }
}

// TODO: tag prog_items and junk_items with proper types
pub fn shuffle_world(
    rng: &mut StdRng,
    mut locations: Vec<Location>,
    mut prog_items: Vec<LabelledItem>,
    mut junk_items: Vec<LabelledItem>
) -> Option<Shuffled> {
    rng.shuffle(&mut prog_items);
    rng.shuffle(&mut locations);
    rng.shuffle(&mut junk_items);
    Shuffled::new(locations, prog_items, junk_items)
}