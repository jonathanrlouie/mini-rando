use amethyst::{
    prelude::*,
    ecs::prelude::Write,
    assets::{Prefab, PrefabLoader, RonFormat, ProgressCounter, Completion, Handle}
};
use super::{
    play::Play,
    prefabs::WasChecked
};
use super::super::super::{
    game_data::{MiniRandoGameData, StateDispatcher},
    randomizer::{
        seed::Seed
    }
};

#[derive(Default)]
pub struct LoadPlay {
    was_checked_handle: Option<Handle<Prefab<WasChecked>>>,
    progress: ProgressCounter,
    seed: Option<Seed>
}

impl LoadPlay {
    pub fn new(seed: Seed) -> Self {
        let mut load_play: LoadPlay = Default::default();
        load_play.seed = Some(seed);
        load_play
    }

    fn load_was_checked_prefab(&mut self, world: &mut World) {
        world.exec(
            |loader: PrefabLoader<WasChecked>| {
            self.was_checked_handle = Some(
                loader.load(
                    "location.ron",
                    RonFormat,
                    (),
                    &mut self.progress
                )
            );
        });
    }

    fn load_complete<'a, 'b>(&mut self) -> Option<Trans<MiniRandoGameData<'a, 'b>, StateEvent>> {
        let handle = self.was_checked_handle.take()?;
        let seed = self.seed.take()?;
        Some(Trans::Switch(
            Box::new(Play { seed, was_checked_handle: handle })
        ))
    }
}

impl <'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for LoadPlay {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        let StateData { world, .. } = data;

        self.load_was_checked_prefab(world);
    }

    fn update(&mut self, data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        data.data.update_no_dispatcher(&data.world);

        match self.progress.complete() {
            Completion::Loading => Trans::None,
            Completion::Complete => self.load_complete().unwrap_or_else(|| Trans::Quit),
            Completion::Failed => Trans::Quit
        }
    }
}