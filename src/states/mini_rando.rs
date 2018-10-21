use amethyst::{
    prelude::*,
    renderer::{Event,VirtualKeyCode},
    input::{is_close_requested, is_key_down},
};

pub struct MiniRando;

impl<'a, 'b> State<GameData<'a, 'b>> for MiniRando {
    fn on_start(&mut self, data: StateData<GameData>) {

    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}