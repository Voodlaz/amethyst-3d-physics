use amethyst::prelude::*;

use amethyst::assets::*;
use amethyst::renderer::*;
use crate::visual_utils;

use amethyst::core::{
    math::Vector3,
    transform::Transform
};

use amethyst_physics::prelude::*;

use amethyst::renderer::shape::Shape;

use amethyst::renderer::rendy::mesh::{
    Position,
    Normal,
    Tangent,
    TexCoord,
};

pub fn init_floor(world: &mut World) {
    let shape = {
        let desc = ShapeDesc::Cube {
            half_extents: Vector3::new(13.0, 0.2, 13.0),
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.mode = BodyMode::Static;

        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    let mesh = {
        let mesh_data: types::MeshData = Shape::Cube
            .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(Some((
                13.0, 0.2, 13.0,
            )))
            .into();

        visual_utils::create_mesh(world, mesh_data)
    };

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, -0.5);
    transform.set_rotation_x_axis(-0.5);    

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .with(shape)
        .with(rb)
        .build();
}
