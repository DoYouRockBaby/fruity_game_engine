use crate::graphics_manager::GraphicsManager;
use crate::image_loader::image_loader;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::world::World;

pub mod graphics_manager;
pub mod image_loader;
pub mod image_resource;

/// Initialize this extension
pub fn initialize(world: &World) {
    let graphics_manager = GraphicsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphics_manager", graphics_manager);

    let resources_manager = service_manager.get::<ResourcesManager>().unwrap();
    let mut resources_manager = resources_manager.write().unwrap();
    resources_manager
        .add_resource_loader("png", image_loader)
        .unwrap();
    resources_manager
        .add_resource_loader("jpg", image_loader)
        .unwrap();
    resources_manager
        .add_resource_loader("gif", image_loader)
        .unwrap();
    resources_manager
        .add_resource_loader("bmp", image_loader)
        .unwrap();
    resources_manager
        .add_resource_loader("ico", image_loader)
        .unwrap();
    resources_manager
        .add_resource_loader("tiff", image_loader)
        .unwrap();
}
