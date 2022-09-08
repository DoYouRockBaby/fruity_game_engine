use crate::javascript_service::JavascriptService;
use crate::js_value::object::JsObject;
use crate::runtime::JsRuntime;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;

mod bridge;
pub mod error;
mod exception;
pub mod javascript_service;
mod js_value;
mod module_map;
mod normalize_path;
mod runtime;
mod serialize;
mod thread_scope_stack;

/// The module name
pub static MODULE_NAME: &str = "fruity_javascript";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let javascript_service = JavascriptService::new(resource_container.clone());

    resource_container.add::<JavascriptService>("javascript_service", Box::new(javascript_service));
}
