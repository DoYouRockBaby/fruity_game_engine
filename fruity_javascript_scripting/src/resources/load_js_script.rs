use crate::JavascriptEngine;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

pub fn load_js_script(
    identifier: ResourceIdentifier,
    _reader: &mut dyn Read,
    _params: Settings,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    let javascript_engine = {
        let service_manager = service_manager.read().unwrap();
        service_manager.get::<JavascriptEngine>().unwrap()
    };

    let javascript_engine = javascript_engine.read().unwrap();
    javascript_engine.run_module(&identifier.0);
}
