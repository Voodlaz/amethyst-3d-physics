use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Srgb;
use amethyst::ecs::{World, WorldExt};
use amethyst::core::Transform;

use amethyst::prelude::*;

pub fn init_light(world: &mut World) {
    let light: Light = PointLight {
        //angle: -0.5,
        intensity: 30.57,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..Default::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 6.0, 2.0);

    world.create_entity()
        .with(light)
        .with(transform)
        .build();
}
