use crate::bridge::service::configure_services;
use crate::error::log_js_error;
use crate::runtime::JsRuntime;
use fruity_core::service_manager::ServiceManager;

mod bridge;
mod error;
mod exception;
mod js_value;
mod module_map;
mod normalize_path;
mod runtime;
mod serialize;
mod value;

pub fn execute_script(service_manager: &mut ServiceManager, path: &str) {
    // Initialize runtime
    let mut runtime = JsRuntime::new();
    configure_services(&mut runtime, service_manager);
    runtime.update_global_bindings();

    // Try module script running
    match runtime.run_module(path) {
        Ok(_) => (),
        Err(err) => log_js_error(&err),
    };
}
