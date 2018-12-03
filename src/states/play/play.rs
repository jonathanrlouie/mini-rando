use amethyst::{
    prelude::*,
    assets::{PrefabLoader, RonFormat}
};
use rand::{SeedableRng, StdRng};
use super::super::super::game_data::{MiniRandoGameData, StateDispatcher};
use super::super::super::randomizer::{
    filler::{FilledLocation, shuffle_and_fill},
    location::{Location, LocId, IsAccessible, has_item},
    item::{Item, LabelledItem}
};
use super::super::SEED_LENGTH;
use super::prefabs::LocationPrefabData;

pub struct Play {
    pub seed: [u8; SEED_LENGTH]
}

impl Play {
    fn generate_locations(&self) -> Vec<FilledLocation> {
        let mut locations: Vec<Location> = vec![
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

        let mut rng: StdRng = StdRng::from_seed(self.seed);

        shuffle_and_fill(&mut rng, locations, prog_items, junk_items)
    }
}

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for Play {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;
        let filled_location = self.generate_locations();
        let mut handle = None;
        world.exec(
            |loader: PrefabLoader<LocationPrefabData>| {
                handle = Some(loader.load("location.ron", RonFormat, (), ()));
            }
        );
        world
            .create_entity()
            .with(handle.unwrap())
            .build();
    }

    fn update(&mut self, data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, StateDispatcher::Play);
        Trans::None
    }
}