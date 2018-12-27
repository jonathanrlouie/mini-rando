use amethyst::{
    prelude::*,
    ecs::prelude::Write,
    assets::{Prefab, PrefabLoader, RonFormat, ProgressCounter, Completion, Handle}
};
use rand::{SeedableRng, StdRng};
use super::super::super::game_data::{MiniRandoGameData, StateDispatcher};
use super::super::super::randomizer::{
    filler::{FilledLocation, shuffle_and_fill},
    location::{Location, LocId, IsAccessible, has_item},
    item::{Item, LabelledItem},
    seed::{Seed}
};
use super::prefabs::WasChecked;

#[derive(Default)]
struct Scene {
    handle: Option<Handle<Prefab<WasChecked>>>
}

pub struct Play {
    pub seed: Seed,
    pub progress: Option<ProgressCounter>,
    pub initialized: bool
}

impl Play {
    fn generate_locations(&self) -> Option<Vec<FilledLocation>> {
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

        let mut rng: StdRng = StdRng::seed_from_u64(self.seed.int_seed.0);

        shuffle_and_fill(&mut rng, locations, prog_items, junk_items)
    }
}

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for Play {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;
        let filled_locations = self.generate_locations();

        self.progress = Some(ProgressCounter::default());

        world.exec(
            |(loader, mut scene): (PrefabLoader<WasChecked>, Write<Scene>)| {
                scene.handle = Some(
                    {
                        loader.load("location.ron", RonFormat, (), self.progress.as_mut().unwrap())
                    });
            }
        );



    }

    fn update(&mut self, data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, StateDispatcher::Play);

        if !self.initialized {
            let remove = match self.progress.as_ref().map(|p| p.complete()) {
                None | Some(Completion::Loading) => false,

                Some(Completion::Complete) => {
                    let scene_handle = data
                        .world
                        .read_resource::<Scene>()
                        .handle
                        .as_ref()
                        .unwrap()
                        .clone();

                    data.world.create_entity().with(scene_handle).build();

                    println!("success");
                    self.initialized = true;

                    true
                }

                Some(Completion::Failed) => {
                    println!("Error: {:?}", self.progress.as_ref().unwrap().errors());
                    return Trans::Quit;
                }
            };
            if remove {
                self.progress = None;
            }
        }

        Trans::None
    }
}