use crate::javascript_engine::JavascriptEngine;
use crate::js_value::object::JsObject;
use crate::resources::load_js_script::load_js_script;
use crate::runtime::JsRuntime;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

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
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let javascript_engine = JavascriptEngine::new(resource_manager.clone());

    resource_manager
        .add::<JavascriptEngine>("javascript_engine", Box::new(javascript_engine))
        .unwrap();

    resource_manager.add_resource_loader("js", load_js_script);
}
