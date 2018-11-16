use amethyst::{
    prelude::*,
};
use super::super::game_data::MiniRandoGameData;

pub struct Play;

impl<'a, 'b> State<MiniRandoGameData<'a, 'b>, StateEvent> for Play {
    fn on_start(&mut self, data: StateData<MiniRandoGameData>) {
        println!("play");
    }

    fn update(&mut self, _data: StateData<MiniRandoGameData>) -> Trans<MiniRandoGameData<'a, 'b>, StateEvent> {
        Trans::None
    }
}