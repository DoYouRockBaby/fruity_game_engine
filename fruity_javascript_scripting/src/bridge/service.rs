use crate::js_value::object::JsObject;
use crate::JsRuntime;
use fruity_ecs::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

pub fn configure_services(runtime: &mut JsRuntime, service_manager: Arc<RwLock<ServiceManager>>) {
    let mut handles = runtime.handles.lock().unwrap();
    let mut global_object = handles.global_object();
    let scope = &mut handles.handle_scope();

    let service_manager_object = JsObject::from_service_manager(scope, service_manager);

    global_object.add_field(scope, "services", service_manager_object);
}
