use crate::bridge::components::configure_components;
use crate::bridge::service::configure_services;
use crate::javascript_engine::JavascriptEngine;
use crate::js_value::object::JsObject;
use crate::resources::script_loader::script_loader;
use crate::runtime::JsRuntime;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::world::World;

mod bridge;
pub mod error;
mod exception;
pub mod javascript_engine;
mod js_value;
mod module_map;
mod normalize_path;
mod resources;
mod runtime;
mod serialize;

/// Initialize this extension
pub fn initialize(world: &World) {
    let javascript_engine = JavascriptEngine::new(world);

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("javascript_engine", javascript_engine);

    let mut resources_manager = service_manager.write::<ResourcesManager>();
    resources_manager
        .add_resource_loader("js", script_loader)
        .unwrap();
}
