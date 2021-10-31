use crate::graphics_manager::GraphicsManager;
use crate::resources::image_resource::load_image;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::world::World;

pub mod graphics_manager;
pub mod math;
pub mod resources;

#[no_mangle]
pub fn initialize(world: &World) {
    let graphics_manager = GraphicsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphics_manager", graphics_manager);

    let mut resources_manager = service_manager.write::<ResourcesManager>();
    resources_manager.add_resource_loader("png", load_image);
    resources_manager.add_resource_loader("jpeg", load_image);
    resources_manager.add_resource_loader("jpg", load_image);
    resources_manager.add_resource_loader("gif", load_image);
    resources_manager.add_resource_loader("bmp", load_image);
    resources_manager.add_resource_loader("ico", load_image);
    resources_manager.add_resource_loader("tiff", load_image);
    resources_manager.add_resource_loader("wgsl", load_shader);
    resources_manager.add_resource_loader("material", load_material);
}
