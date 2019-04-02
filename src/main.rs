#[macro_use]
extern crate amethyst;
extern crate tiled;
extern crate rand;
extern crate linked_hash_set;
#[macro_use]
extern crate serde;
extern crate num_traits;
extern crate genmesh;

mod randomizer;
mod states;
mod game_data;
mod tilemap;
mod rng;

use std::time::Duration;

use amethyst::{
    prelude::*,
    core::{
        transform::TransformBundle,
        frame_limiter::FrameRateLimitStrategy
    },
    assets::{
        PrefabLoaderSystem
    },
    renderer::{
        Pipeline, Stage, RenderBundle, DisplayConfig,
    },
    ui::{DrawUi, UiBundle},
    input::InputBundle,
    utils::{
        application_root_dir
    }
};
use self::states::main_menu::{MainMenu};
use self::states::custom_game::input::UiEventHandlerSystem;
use self::states::play::prefabs::ItemLocation;
use self::game_data::{MiniRandoGameDataBuilder};

const FRAME_LIMIT: u32 = 60;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let display_config_path = format!("{}/resources/display.ron", app_root);

    let assets_dir = format!("{}/assets", app_root);

    let key_bindings_path = format!("{}/resources/input.ron", app_root);

    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawUi::new()),
    );

    let game_data = MiniRandoGameDataBuilder::default()
        .with_base_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with_base_bundle(TransformBundle::new())?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_base_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_core(
            PrefabLoaderSystem::<ItemLocation>::default(),
            "item_location_loader",
            &[],
        )
        .with_custom_game(UiEventHandlerSystem::new(), "ui_event_handler", &[]);

    let mut game = Application::build(assets_dir, MainMenu)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
