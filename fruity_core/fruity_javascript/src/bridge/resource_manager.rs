use crate::js_value::object::JsObject;
use crate::JsRuntime;
use fruity_core::resource::resource_manager::ResourceManager;
use std::sync::Arc;

pub static RESOURCE_MANAGER_GLOBAL_VAR_NAME: &str = "resourceManager";

pub fn configure_resource_manager(runtime: &mut JsRuntime, resource_manager: Arc<ResourceManager>) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let resource_manager_object =
        JsObject::from_introspect_object(scope, Box::new(resource_manager));

    global_object.add_field_with_raw_name(
        scope,
        RESOURCE_MANAGER_GLOBAL_VAR_NAME,
        resource_manager_object,
    );
}
