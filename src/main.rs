use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod space_shooter_types;

struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default().with_bundle(RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.034, 0.036, 0.052, 1.0]),)
                .with_plugin(RenderFlat3D::default()),)?
                    .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}