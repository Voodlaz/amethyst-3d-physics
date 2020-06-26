use amethyst::prelude::*;

use amethyst::assets::*;
use amethyst::renderer::*;

pub fn create_mesh(world: &World, mesh_data: types::MeshData) -> Handle<types::Mesh> {
    // Mesh creation
    let loader = world.read_resource::<Loader>();
    let asset_storage = world.read_resource::<AssetStorage<types::Mesh>>();

    loader.load_from_data(mesh_data, (), &asset_storage)
}
