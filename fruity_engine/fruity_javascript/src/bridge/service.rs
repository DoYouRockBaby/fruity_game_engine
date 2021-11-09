use crate::js_value::object::JsObject;
use crate::JsRuntime;
use fruity_core::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

pub static SERVICE_MANAGER_GLOBAL_VAR_NAME: &str = "services";

pub fn configure_services(runtime: &mut JsRuntime, service_manager: Arc<RwLock<ServiceManager>>) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let service_manager_object = JsObject::from_service_manager(scope, service_manager);

    global_object.add_field(
        scope,
        SERVICE_MANAGER_GLOBAL_VAR_NAME,
        service_manager_object,
    );
}
