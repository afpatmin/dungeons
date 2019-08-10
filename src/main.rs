extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod states;
mod systems;

use states::Game;
use systems::MoveSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path).with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
        .with_plugin(RenderFlat2D::default());

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::MoveSystem, "move_system", &["input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Game, game_data)?;
    game.run();

    Ok(())
}
