use crate::service1::Service1;
use crate::JsRuntime;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_introspect::IntrospectMethods;
use std::sync::Arc;
use std::sync::RwLock;

pub fn configure_services(runtime: &mut JsRuntime, _service_manager: &ServiceManager) {
    let service1 = Arc::new(RwLock::new(
        Box::new(Service1::new()) as Box<dyn IntrospectMethods + Send + Sync>
    )) as Arc<RwLock<Box<dyn IntrospectMethods + Send + Sync>>>;

    let global_object = runtime.global_object();
    global_object.add_object_from_introspect("service1", service1);
}
