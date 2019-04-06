use rand::{Rng, StdRng};
use super::{
    item::LabelledItem,
    location::Location
};
pub use self::shuffled::Shuffled;
use super::super::rng::GameRng;

pub mod shuffled {
    use super::{
        LabelledItem,
        Location
    };

    pub struct Shuffled<F: Fn(&[LabelledItem]) -> bool>(Vec<Location<F>>, Vec<LabelledItem>, Vec<LabelledItem>);

    impl<F: Fn(&[LabelledItem]) -> bool> Shuffled<F> {
        pub fn new(locations: Vec<Location<F>>, prog_items: Vec<LabelledItem>, junk_items: Vec<LabelledItem>) -> Option<Self> {
            if locations.len() == prog_items.len() + junk_items.len() {
                Some(Shuffled(locations, prog_items, junk_items))
            } else {
                None
            }
        }

        pub fn get(self) -> (Vec<Location<F>>, Vec<LabelledItem>, Vec<LabelledItem>) {
            (self.0, self.1, self.2)
        }
    }
}

pub fn shuffle_world<F: Fn(&[LabelledItem]) -> bool>(
    rng: &mut GameRng,
    locations: Vec<Location<F>>,
    prog_items: Vec<LabelledItem>,
    junk_items: Vec<LabelledItem>
) -> Option<Shuffled<F>> {
    Shuffled::new(
        rng.shuffle(locations),
        rng.shuffle(prog_items),
        rng.shuffle(junk_items))
}