extern crate amethyst;
extern crate tiled;
extern crate rand;

mod randomizer;
mod states;

use std::time::Duration;

use amethyst::{
    prelude::*,
    core::frame_limiter::FrameRateLimitStrategy,
    renderer::{
        DrawFlat,
        PosTex
    }
};
use states::mini_rando::MiniRando;

const FRAME_LIMIT: u32 = 60;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config_path = format!(
        "{}/resources/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let assets_dir = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new(), true)?;

    let mut game = Application::build(assets_dir, MiniRando)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
