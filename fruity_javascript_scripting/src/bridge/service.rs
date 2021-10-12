use crate::JsRuntime;
use fruity_core::service_manager::ServiceManager;

pub fn configure_services(runtime: &mut JsRuntime, service_manager: &ServiceManager) {
    let global_object = runtime.global_object();
    service_manager.iter().for_each(|service| {
        global_object.add_service("service1", service);
    })
}
