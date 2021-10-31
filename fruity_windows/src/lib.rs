use crate::windows_manager::WindowsManager;
use fruity_core::world::World;

pub mod windows_manager;

#[no_mangle]
pub fn initialize(world: &World) {
    let windows_manager = WindowsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("windows_manager", windows_manager);

    let service_manager = world.service_manager.clone();
    world.set_run_callback(move || {
        let service_manager = service_manager.clone();
        let service_manager = service_manager.read().unwrap();
        let windows_manager = service_manager.read::<WindowsManager>();

        windows_manager.run();
    })
}
