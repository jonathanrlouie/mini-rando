extern crate amethyst;
extern crate tiled;
extern crate rand;
extern crate linked_hash_set;

mod randomizer;
mod states;

use std::time::Duration;

use amethyst::{
    prelude::*,
    core::{
        transform::TransformBundle,
        frame_limiter::FrameRateLimitStrategy
    },
    renderer::{
        DrawFlat, PosTex, Pipeline, Stage, DrawSprite, RenderBundle, DisplayConfig,
    },
    ui::{DrawUi, UiBundle},
    input::InputBundle,
    utils::{
        application_root_dir
    }
};
use states::main_menu::MainMenu;

const FRAME_LIMIT: u32 = 60;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let display_config_path = format!("{}/resources/display.ron", app_root);

    let assets = format!("{}/assets", app_root);

    let key_bindings_path = format!("{}/resources/input.ron", app_root);

    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawSprite::new())
            .with_pass(DrawUi::new()),
    );

    let assets_dir = format!("{}/resources", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;

    let mut game = Application::build(assets_dir, MainMenu)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
