mod systems;
mod catvolleyball;

use crate::catvolleyball::CatVolleyball;
use amethyst::{
    prelude::*,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
};


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = app_root.join("resources")
        .join("bindings_config.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;


    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::MoveBallSystem, "ball_system", &[])
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"])
        .with(
            systems::CollisionSystem,
            "collision_system",
            &["player_system", "ball_system"],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::new(assets_dir, CatVolleyball, game_data)?;
    game.run();

    Ok(())
}
