use amethyst::{
    prelude::*,
    ecs::prelude::{Write, Entity},
    assets::{Prefab, PrefabLoader, RonFormat, ProgressCounter, Completion, Handle}
};
use rand::{SeedableRng, StdRng};
use super::super::super::rng::GameRng;
use super::super::super::game_data::{MiniRandoGameData, StateDispatcher};
use super::super::super::randomizer::{
    filler::{FilledLocation, shuffle_and_fill},
    location::{Location, LocId, IsAccessible, has_item},
    item::{Item, LabelledItem},
    seed::{Seed}
};
use super::prefabs::{ItemLocation, WasChecked};

pub struct Play {
    pub rng: GameRng,
    pub item_location_handle: Handle<Prefab<ItemLocation>>,
    entity: Option<Entity>
}

impl Play {
    pub fn new(rng: GameRng, item_location_handle: Handle<Prefab<ItemLocation>>) -> Self {
        Self {
            rng,
            item_location_handle,
            entity: None
        }
    }

    fn generate_locations(&mut self) -> Option<Vec<FilledLocation>> {
        let locations: Vec<Location<fn(&[LabelledItem]) -> bool>> = vec![
            Location(LocId::Loc0, IsAccessible(||
                |items| has_item(items, LabelledItem::Progression(Item::Item0)))),
            Location(LocId::Loc1, IsAccessible(|| |items| {
                has_item(items, LabelledItem::Progression(Item::Item0)) &&
                    has_item(items, LabelledItem::Progression(Item::Item1))
            })),
            Location(LocId::Loc2, IsAccessible(|| |_| true)),
            Location(LocId::Loc3, IsAccessible(|| |_| true)),
            Location(LocId::Loc4, IsAccessible(|| |_| true)),
            Location(LocId::Loc5, IsAccessible(|| |_| true))
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

        shuffle_and_fill(&mut self.rng, locations, prog_items, junk_items)
    }
}

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for Play {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;
        let filled_locations = self.generate_locations();
        self.entity = Some(world
            .create_entity()
            .with(self.item_location_handle.clone())
            .build());
    }

    fn update(&mut self, data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, StateDispatcher::Play);

        let storage = data.world.read_storage::<WasChecked>();

        let was_checked = storage.get(self.entity.unwrap()).expect("couldn't get was checked");

        println!("{:?}", was_checked.was_checked);

        Trans::None
    }
}