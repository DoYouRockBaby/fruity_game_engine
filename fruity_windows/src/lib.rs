#![crate_type = "cdylib"]
use crate::windows_manager::WindowsManager;
use fruity_core::world::World;

pub mod windows_manager;

pub fn identifier() -> &'static str {
    "fruity_windows"
}

pub fn dependencies() -> &'static [&'static str] {
    &[]
}

pub fn initialize(world: &World) {
    let windows_manager = WindowsManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("windows_manager", windows_manager);
}

pub fn run(world: &World) {
    let service_manager = world.service_manager.read().unwrap();
    let windows_manager = service_manager.read::<WindowsManager>();
    windows_manager.run();
}
