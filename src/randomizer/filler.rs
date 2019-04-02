use linked_hash_set::LinkedHashSet;
use std::iter::FromIterator;
use rand::StdRng;
use super::{
    item::LabelledItem,
    location::{Location, LocId},
    shuffler::{shuffle_world, Shuffled},
};
use super::super::rng::GameRng;

pub use self::fast_filler_args::FastFillerArgs;

#[derive(Debug, PartialEq)]
pub struct FilledLocation(pub LabelledItem, pub LocId);

// Contains filled locations and remaining empty locations
struct ProgressionFillerResult(Vec<FilledLocation>, LinkedHashSet<LocId>);

pub mod fast_filler_args {
    use linked_hash_set::LinkedHashSet;
    use super::super::{
        item::LabelledItem,
        location::LocId
    };

    pub struct FastFillerArgs(Vec<LabelledItem>, LinkedHashSet<LocId>);

    impl FastFillerArgs {
        pub fn new(items: Vec<LabelledItem>, locations: LinkedHashSet<LocId>) -> Option<Self> {
            if items.len() == locations.len() {
                Some(FastFillerArgs(items, locations))
            } else {
                None
            }
        }

        pub fn get(self) -> (Vec<LabelledItem>, LinkedHashSet<LocId>) {
            (self.0, self.1)
        }
    }

}

pub fn shuffle_and_fill(
    rng: &mut GameRng,
    locations: Vec<Location>,
    prog_items: Vec<LabelledItem>,
    junk_items: Vec<LabelledItem>
) -> Option<Vec<FilledLocation>> {
    let shuffled = shuffle_world(rng, locations, prog_items, junk_items)?;
    fill_locations(shuffled)
}

fn fill_locations(
    shuffled: Shuffled
) -> Option<Vec<FilledLocation>> {
    let (locations, prog_items, other_items) = shuffled.get();

    let ProgressionFillerResult(mut filled_locs, remaining_locs): ProgressionFillerResult =
        progression_filler(prog_items, locations)?;

    let option_fast_filler_args = FastFillerArgs::new(other_items, remaining_locs);

    if let Some(fast_filler_args) = option_fast_filler_args {
        filled_locs.append(&mut fast_filler(fast_filler_args));
        Some(filled_locs)
    } else {
        None
    }
}

fn progression_filler(
    mut prog_items: Vec<LabelledItem>,
    mut locations: Vec<Location>
) -> Option<ProgressionFillerResult> {
    let mut remaining_locations: LinkedHashSet<LocId> =
        LinkedHashSet::from_iter(locations
            .iter()
            .map(|loc| (*loc).0));

    let mut filled_locations: Vec<FilledLocation> = vec![];

    let item_count = prog_items.len();

    for _ in 0..item_count {
        let item = prog_items.pop()?;
        locations = locations
            .into_iter()
            .filter(|&Location(_, ref is_accessible)| is_accessible.0(&prog_items))
            .collect();
        let location = locations.pop()?;
        filled_locations.push(FilledLocation(item, location.0));
        remaining_locations.remove(&location.0);
    }

    Some(ProgressionFillerResult(filled_locations, remaining_locations))
}

fn fast_filler(args: FastFillerArgs) -> Vec<FilledLocation> {
    let (items, locations) = args.get();
    items
        .into_iter()
        .zip(locations)
        .map(|(item, loc)| FilledLocation(item, loc))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::item::Item;
    use super::super::location::{has_item, IsAccessible};
    use super::super::seed::Seed;
    use rand::{StdRng, SeedableRng};
    use super::super::super::rng::GameRng;

    // TODO: Add more tests
    #[test]
    fn filler_test() {
        for _ in 0..10 {
            let locations: Vec<Location> = vec![
                Location(LocId::Loc0, IsAccessible(Box::new(
                    |items| has_item(items, LabelledItem::Progression(Item::Item0))))),
                Location(LocId::Loc1, IsAccessible(Box::new(|items| {
                    has_item(items, LabelledItem::Progression(Item::Item0)) &&
                        has_item(items, LabelledItem::Progression(Item::Item1))
                }))),
                Location(LocId::Loc2, IsAccessible(Box::new(|_| true))),
                Location(LocId::Loc3, IsAccessible(Box::new(|_| true))),
                Location(LocId::Loc4, IsAccessible(Box::new(|_| true))),
                Location(LocId::Loc5, IsAccessible(Box::new(|_| true)))
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

            let mut rng: GameRng = GameRng::new(Seed::generate_seed()
                .expect("Seed failed to generate."));

            let filled_locations =
                shuffle_and_fill(&mut rng, locations, prog_items, junk_items)
                    .expect("Number of locations does not match total number of items.");

            assert_eq!(filled_locations.len(), 6);

            println!("{:?}", filled_locations);

            // check that items were not placed in certain locations
            assert!(!filled_locations
                .iter()
                .any(|filled_loc|
                    filled_loc == &FilledLocation(
                        LabelledItem::Progression(Item::Item0),
                        LocId::Loc0
                    ) || filled_loc == &FilledLocation(
                        LabelledItem::Progression(Item::Item0),
                        LocId::Loc1
                    ) || filled_loc == &FilledLocation(
                        LabelledItem::Progression(Item::Item1),
                        LocId::Loc1
                    )
                ));
        }
    }
}