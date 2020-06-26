use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};


mod visual_utils;
mod objects;
use crate::objects::{camera, light, floor, cube};

use amethyst_physics::PhysicsBundle;
use amethyst_nphysics::NPhysicsBackend;

struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        camera::init_camera(data.world);
        light::init_light(data.world);
        cube::init_cube(data.world);
        cube::init_cube_2(data.world);
        floor::init_floor(data.world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.95, 0.95, 0.95, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
