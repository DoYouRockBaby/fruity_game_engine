use crate::graphics_manager::GraphicsManager;
use fruity_ecs::world::World;
use fruity_windows::windows_manager::WindowsManager;

pub mod graphics_manager;

/// Initialize this extension
pub fn initialize(world: &World) {
    let mut service_manager = world.service_manager.write().unwrap();
    let windows_manager = service_manager.get::<WindowsManager>().unwrap();

    service_manager.register("graphics_manager", GraphicsManager::new(windows_manager));
}
