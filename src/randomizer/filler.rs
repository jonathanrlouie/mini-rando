use rand::{Rng, SeedableRng, StdRng};
use super::FilledLocation;
use super::item::{LabelledItem, Item};
use super::location::{Location};

struct ProgressionFillerResult(Vec<FilledLocation>, Vec<Location>);

pub fn fill_locations(mut locations: Vec<Location>, mut prog_items: Vec<LabelledItem>, mut other_items: Vec<LabelledItem>) -> () {
    let mut rng = StdRng::from_seed([0u8; 32]);

    rng.shuffle(&mut prog_items);
    rng.shuffle(&mut locations);
    rng.shuffle(&mut other_items);

    let filled_locations: ProgressionFillerResult = progression_filler(prog_items, locations);

    println!("{:?}, {:?}, {:?}", filled_locations.0[0], filled_locations.0[1], filled_locations.0[2]);
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

pub fn fast_filler(items: Vec<LabelledItem>, locations: Vec<Location>) -> Vec<FilledLocation> {
    items
        .into_iter()
        .zip(locations)
        .map(|(item, loc)| FilledLocation(item, loc))
        .collect()
}