use linked_hash_set::LinkedHashSet;
use std::iter::FromIterator;
use super::{
    item::LabelledItem,
    location::{Location, LocId}
};

#[derive(Debug, PartialEq)]
pub struct FilledLocation(pub LabelledItem, pub LocId);

// Contains filled locations and remaining empty locations
struct ProgressionFillerResult(Vec<FilledLocation>, LinkedHashSet<LocId>);

pub fn fill_locations(
    locations: Vec<Location>,
    prog_items: Vec<LabelledItem>,
    other_items: Vec<LabelledItem>
) -> Vec<FilledLocation> {
    debug_assert!(locations.len() == prog_items.len() + other_items.len());

    let ProgressionFillerResult(mut filled_locs, remaining_locs): ProgressionFillerResult =
        progression_filler(prog_items, locations);

    filled_locs.append(&mut fast_filler(other_items, remaining_locs));
    filled_locs
}

fn progression_filler(
    mut prog_items: Vec<LabelledItem>,
    locations_vec: Vec<Location>
) -> ProgressionFillerResult {
    let mut remaining_locations: LinkedHashSet<LocId> =
        LinkedHashSet::from_iter(locations_vec
            .iter()
            .map(|ref loc| loc.0));

    let mut locations: LinkedHashSet<Location> =
        LinkedHashSet::from_iter(locations_vec.into_iter());

    let mut filled_locations: Vec<FilledLocation> = vec![];

    let item_count = prog_items.len();

    for _ in 0..item_count {
        let option_item = prog_items.pop();
        locations = locations
            .into_iter()
            .filter(|&Location(_, ref is_accessible)| is_accessible.0(&prog_items))
            .collect();
        let option_location = locations.pop_front();
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
    use rand::{Rng, StdRng, SeedableRng};

    #[test]
    fn filler_test() {
        let mut locations: Vec<Location> = vec![
            Location(LocId(0), IsAccessible(Box::new(
                |items| has_item(items, LabelledItem::Progression(Item::Item0))))),
            Location(LocId(1), IsAccessible(Box::new(|items| {
                has_item(items, LabelledItem::Progression(Item::Item0)) &&
                    has_item(items, LabelledItem::Progression(Item::Item1))
            }))),
            Location(LocId(2), IsAccessible(Box::new(|_| true))),
            Location(LocId(3), IsAccessible(Box::new(|_| true))),
            Location(LocId(4), IsAccessible(Box::new(|_| true))),
            Location(LocId(5), IsAccessible(Box::new(|_| true)))
        ];

        let mut prog_items: Vec<LabelledItem> = vec![
            LabelledItem::Progression(Item::Item0),
            LabelledItem::Progression(Item::Item1),
            LabelledItem::Progression(Item::Item2)
        ];

        let mut junk_items: Vec<LabelledItem> = vec![
            LabelledItem::Junk(Item::Item3),
            LabelledItem::Junk(Item::Item3),
            LabelledItem::Junk(Item::Item3)
        ];

        let mut rng: StdRng = StdRng::from_seed([0u8; 32]);

        rng.shuffle(&mut prog_items);
        rng.shuffle(&mut locations);
        rng.shuffle(&mut junk_items);

        let filled_locations =
            fill_locations(
                locations,
                prog_items,
                junk_items);

        assert_eq!(filled_locations.len(), 6);

        println!("{:?}", filled_locations);

        // check that items were not placed in certain locations
        assert!(!filled_locations
            .iter()
            .any(|filled_loc|
                filled_loc == &FilledLocation(
                    LabelledItem::Progression(Item::Item0),
                    LocId(0)
                ) || filled_loc == &FilledLocation(
                    LabelledItem::Progression(Item::Item0),
                    LocId(1)
                ) ||
                filled_loc == &FilledLocation(
                    LabelledItem::Progression(Item::Item1),
                    LocId(1)
                )
            ));
    }
}