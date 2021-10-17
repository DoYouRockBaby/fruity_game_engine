use crate::graphics_manager::GraphicsManager;
use fruity_ecs::world::World;

pub mod graphics_manager;

/// Initialize this extension
pub fn initialize(world: &World) {
    let graphics_manager = GraphicsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("graphics_manager", graphics_manager);
}
