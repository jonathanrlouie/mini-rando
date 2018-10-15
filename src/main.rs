extern crate amethyst;
extern crate tiled;
extern crate rand;

mod randomizer;
mod states;

use amethyst::{
    prelude::*,
};
use states::mini_rando::MiniRando;

const FRAME_LIMIT: u32 = 60;

fn main() {
    //let assets_dir = format!("{}/resources", application_root_dir());

    //let mut game = Application::build(assets_dir, MiniRando);
}
