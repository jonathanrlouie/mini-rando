use linked_hash_set::LinkedHashSet;
use std::iter::FromIterator;
use rand::StdRng;
use super::{
    item::LabelledItem,
    location::{Location, LocId},
    shuffler::{shuffle_world, Shuffled},
};

#[derive(Debug, PartialEq)]
pub struct FilledLocation(pub LabelledItem, pub LocId);

// Contains filled locations and remaining empty locations
struct ProgressionFillerResult(Vec<FilledLocation>, LinkedHashSet<LocId>);

pub fn shuffle_and_fill(
    rng: &mut StdRng,
    locations: Vec<Location>,
    prog_items: Vec<LabelledItem>,
    junk_items: Vec<LabelledItem>
) -> Option<Vec<FilledLocation>> {
    shuffle_world(rng, locations, prog_items, junk_items)
        .map(fill_locations)
}

fn fill_locations(
    shuffled: Shuffled
) -> Vec<FilledLocation> {
    let (locations, prog_items, other_items) = shuffled.get();

    let ProgressionFillerResult(mut filled_locs, remaining_locs): ProgressionFillerResult =
        progression_filler(prog_items, locations);

    filled_locs.append(&mut fast_filler(other_items, remaining_locs));
    filled_locs
}

fn progression_filler(
    mut prog_items: Vec<LabelledItem>,
    mut locations: Vec<Location>
) -> ProgressionFillerResult {
    let mut remaining_locations: LinkedHashSet<LocId> =
        LinkedHashSet::from_iter(locations
            .iter()
            .map(|ref loc| loc.0));

    let mut filled_locations: Vec<FilledLocation> = vec![];

    let item_count = prog_items.len();

    for _ in 0..item_count {
        let option_item = prog_items.pop();
        locations = locations
            .into_iter()
            .filter(|&Location(_, ref is_accessible)| is_accessible.0(&prog_items))
            .collect();
        let option_location = locations.pop();
        if let (Some(item), Some(chosen_location)) = (option_item, option_location) {
            filled_locations.push(FilledLocation(item, chosen_location.0));
            remaining_locations.remove(&chosen_location.0);
        } else if option_item.is_none() {
            panic!("Out of items");
        } else {
            panic!("Out of locations");
        }
    }

    ProgressionFillerResult(filled_locations, remaining_locations)
}

fn fast_filler(items: Vec<LabelledItem>, locations: LinkedHashSet<LocId>) -> Vec<FilledLocation> {
    debug_assert!(items.len() == locations.len());
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

            let mut rng: StdRng = StdRng::seed_from_u64(Seed::generate_seed().int_seed.0);

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