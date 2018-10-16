use amethyst::{
    prelude::*,
    renderer::Event,
};

pub struct MiniRando;

impl<'a, 'b> State<GameData<'a, 'b>> for MiniRando {
    fn on_start(&mut self, data: StateData<GameData>) {

    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {

        Trans::None

    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        Trans::None
    }
}