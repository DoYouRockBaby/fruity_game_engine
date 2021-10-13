use crate::bridge::service::configure_services;
use crate::js_value::object::JsObject;
use crate::runtime::JsRuntime;
use fruity_ecs::world::World;

mod bridge;
pub mod error;
mod exception;
mod js_value;
mod module_map;
mod normalize_path;
pub mod runtime;
mod serialize;

/// Initialize this extension
pub fn initialize(world: &World) {
    let mut runtime = JsRuntime::new();
    configure_services(&mut runtime, world);
    runtime.update_global_bindings();

    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("js_runtime", runtime);
}
