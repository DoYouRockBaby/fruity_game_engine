use crate::windows_manager::WindowsManager;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::world::World;

pub mod windows_manager;

/// Initialize this extension
pub fn initialize(world: &World) {
    let mut service_manager = world.service_manager.write().unwrap();
    let system_manager = service_manager.get::<SystemManager>().unwrap();

    service_manager.register("windows_manager", WindowsManager::new(system_manager));
}
