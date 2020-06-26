use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::core::transform::*;
use amethyst::window::ScreenDimensions;

pub fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 17.0, 6.0);
    transform.set_rotation_x_axis(5.0);

    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world.create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}
