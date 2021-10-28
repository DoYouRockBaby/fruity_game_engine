use crate::components::controls::Controls;
use crate::editor_manager::EditorManager;
use fruity_core::world::World;

pub mod components;
pub mod editor_manager;
pub mod state;
pub mod style;

/// Initialize this extension
pub fn initialize(world: &World) {
    let editor_manager = EditorManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("editor_manager", editor_manager);
}
