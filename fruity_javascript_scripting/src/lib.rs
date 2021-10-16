use crate::bridge::components::configure_components;
use crate::bridge::service::configure_services;
use crate::javascript_engine::JavascriptEngine;
use crate::js_value::object::JsObject;
use crate::runtime::JsRuntime;
use fruity_ecs::world::World;

mod bridge;
pub mod error;
mod exception;
pub mod javascript_engine;
mod js_value;
mod module_map;
mod normalize_path;
mod runtime;
mod serialize;

/// Initialize this extension
pub fn initialize(world: &World) {
    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("javascript_engine", JavascriptEngine::new(world));
}
