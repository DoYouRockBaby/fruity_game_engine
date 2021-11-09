use crate::bridge::constructors::configure_constructors;
use crate::bridge::service::configure_services;
use crate::javascript_engine::JavascriptEngine;
use crate::js_value::object::JsObject;
use crate::resources::load_js_script::load_js_script;
use crate::runtime::JsRuntime;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;
use std::sync::RwLock;

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
mod thread_scope_stack;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let javascript_engine = JavascriptEngine::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("javascript_engine", javascript_engine);

    let mut resources_manager = service_manager_writer.write::<ResourcesManager>();
    resources_manager.add_resource_loader("js", load_js_script);
}