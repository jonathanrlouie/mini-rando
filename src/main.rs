extern crate amethyst;
extern crate tiled;

const FRAME_LIMIT: u32 = 60;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use pong::Pong;

    let display_config_path = format!(
        "{}/examples/pong/resources/display.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            format!(
                "{}/examples/pong/resources/input_controller.ron",
                env!("CARGO_MANIFEST_DIR")
            )
        } else {
            format!(
                "{}/examples/pong/resources/input.ron",
                env!("CARGO_MANIFEST_DIR")
            )
        }
    };

    let assets_dir = format!("{}/examples/assets/", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with_bundle(PongBundle)?
        .with_bundle(TransformBundle::new().with_dep(&["ball_system", "paddle_system"]))?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new(), true)?;
    let mut game = Application::build(assets_dir, Pong)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;
    game.run();
    Ok(())
}