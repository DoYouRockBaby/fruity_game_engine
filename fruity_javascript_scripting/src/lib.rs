use crate::bridge::service::configure_services;
use crate::error::log_js_error;
use crate::js_value::object::JsObject;
use crate::runtime::JsRuntime;
use fruity_ecs::world::World;

mod bridge;
mod error;
mod exception;
mod js_value;
mod module_map;
mod normalize_path;
mod runtime;
mod serialize;
mod value;

pub fn execute_script(world: &World, path: &str) {
    // Initialize runtime
    let mut runtime = JsRuntime::new();
    configure_services(&mut runtime, world);

    // Test
    //let global_object = runtime.global_object();
    //global_object.add_field("component1", JsObject::from_component(test));
    //global_object.add_field("component1_mut", JsObject::from_component_mut(test));
    runtime.update_global_bindings();

    // Try module script running
    match runtime.run_module(path) {
        Ok(_) => (),
        Err(err) => log_js_error(&err),
    };
}
