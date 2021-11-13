use crate::javascript_service::JavascriptService;
use crate::js_value::object::JsObject;
use crate::resources::load_js_script::load_js_script;
use crate::runtime::JsRuntime;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

mod bridge;
pub mod error;
mod exception;
pub mod javascript_service;
mod js_value;
mod module_map;
mod normalize_path;
mod resources;
mod runtime;
mod serialize;
mod thread_scope_stack;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let javascript_service = JavascriptService::new(resource_container.clone());

    resource_container
        .add_require::<JavascriptService>("javascript_service", Box::new(javascript_service))
        .unwrap();

    resource_container.add_resource_loader("js", load_js_script);
}
