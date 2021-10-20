use crate::JavascriptEngine;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourceLoaderParams;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_manager::ServiceManager;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

pub fn script_loader(
    _resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    _reader: &mut dyn Read,
    _params: ResourceLoaderParams,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    let javascript_engine = {
        let service_manager = service_manager.read().unwrap();
        service_manager.get::<JavascriptEngine>().unwrap()
    };

    let javascript_engine = javascript_engine.read().unwrap();
    javascript_engine.run_module(&identifier.0);
}
