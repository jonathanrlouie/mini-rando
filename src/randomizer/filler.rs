use rand::{Rng, StdRng};
use super::item::{LabelledItem};
use super::location::{Location};

#[derive(Debug, PartialEq)]
pub struct FilledLocation(pub LabelledItem, pub Location);

// Contains filled locations and remaining empty locations
struct ProgressionFillerResult(Vec<FilledLocation>, Vec<Location>);

pub fn fill_locations(
    mut rng: StdRng,
    mut locations: Vec<Location>,
    mut prog_items: Vec<LabelledItem>,
    mut other_items: Vec<LabelledItem>
) -> Vec<FilledLocation> {
    debug_assert!(locations.len() == prog_items.len() + other_items.len());

    rng.shuffle(&mut prog_items);
    rng.shuffle(&mut locations);
    rng.shuffle(&mut other_items);

    let ProgressionFillerResult(mut filled_locs, remaining_locs): ProgressionFillerResult =
        progression_filler(prog_items, locations);

    filled_locs.append(&mut fast_filler(other_items, remaining_locs));
    filled_locs
}

fn progression_filler(
    mut prog_items: Vec<LabelledItem>,
    mut locations: Vec<Location>
) -> ProgressionFillerResult {
    let mut remaining_locations = locations.clone();

    let mut filled_locations: Vec<FilledLocation> = vec![];

    let item_count = prog_items.len();

    for _ in 0..item_count {
        let option_item = prog_items.pop();
        locations = locations
            .into_iter()
            .filter(|&loc| loc.is_accessible(&prog_items))
            .collect();
        let option_location = locations.pop();
        if let (Some(item), Some(chosen_location)) = (option_item, option_location) {
            filled_locations.push(FilledLocation(item, chosen_location));
            remaining_locations = remaining_locations
                .into_iter()
                .filter(|&loc| loc != chosen_location)
                .collect();
        }
    }

    ProgressionFillerResult(filled_locations, remaining_locations)
}

fn fast_filler(items: Vec<LabelledItem>, locations: Vec<Location>) -> Vec<FilledLocation> {
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
    use rand::SeedableRng;

    #[test]
    fn filler_test() {
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

        assert_eq!(filled_locations.len(), 6);

        assert!(!filled_locations
            .iter()
            .any(|filled_loc| filled_loc == &FilledLocation(LabelledItem::Progression(Item::Item0), Location::Location0)));

        assert!(!filled_locations
            .iter()
            .any(|filled_loc| filled_loc == &FilledLocation(LabelledItem::Progression(Item::Item0), Location::Location1)));

        assert!(!filled_locations
            .iter()
            .any(|filled_loc| filled_loc == &FilledLocation(LabelledItem::Progression(Item::Item1), Location::Location1)));
    }
}