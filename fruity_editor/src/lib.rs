use crate::components::panes::Panes;
use crate::editor_manager::EditorManager;
use fruity_core::world::World;

#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod editor_manager;
pub mod hooks;
pub mod state;
pub mod style;
pub mod ui_element;

#[no_mangle]
pub fn initialize(world: &World) {
    let editor_manager = EditorManager::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("editor_manager", editor_manager);
}