use amethyst::{
    prelude::*,
    utils::application_root_dir,
};

mod components;
mod state;
mod systems;
mod bundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let bindings_config_path = config_dir.join("bindings.ron");

    let main_bundle = bundle::StartingBundle {
        bindings_config_path, display_config_path
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(main_bundle)?
        .with(systems::VelocityUpdateSystem, "velocity_update_system", &[])
        .with(
            systems::VelocityCapperSystem,
            "velocity_capper_system",
            &["velocity_update_system"],
        )
        .with(
            systems::FrictionApplyingSystem,
            "friction_applying_systemm",
            &["velocity_update_system"],
        )
        .with(
            systems::PositionUpdateSystem,
            "position_update_system",
            &["velocity_update_system", "velocity_capper_system"],
        )
        .with_thread_local(systems::PointerUpdateSystem);

    let mut game = Application::new(assets_dir, state::MyState, game_data)?;
    game.run();

    Ok(())
}
