use crate::resources::mesh_resource::MeshResource;
use crate::resources::mesh_resource::Vertex;
use fruity_core::resource::resource_container::ResourceContainer;
use std::sync::Arc;

pub fn load_default_resources(resource_container: Arc<ResourceContainer>) {
    load_squad_mesh(resource_container.clone());
}

pub fn load_squad_mesh(resource_container: Arc<ResourceContainer>) {
    let resource = MeshResource {
        vertices: vec![
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
        ],
    };

    resource_container
        .add("Meshes/Squad", Box::new(resource))
        .unwrap();
}
