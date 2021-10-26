use crate::windows_manager::WindowsManager;
use fruity_core::world::World;

pub mod windows_manager;

/// Initialize this extension
pub fn initialize(world: &World) {
    let windows_manager = WindowsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("windows_manager", windows_manager);
}
