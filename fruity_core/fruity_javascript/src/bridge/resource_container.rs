use crate::js_value::object::JsObject;
use crate::JsRuntime;
use fruity_core::resource::resource_container::ResourceContainer;

pub static RESOURCE_MANAGER_GLOBAL_VAR_NAME: &str = "resourceContainer";

pub fn configure_resource_container(
    runtime: &mut JsRuntime,
    resource_container: ResourceContainer,
) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let resource_container_object =
        JsObject::from_introspect_object(scope, Box::new(resource_container));

    global_object.add_field_with_raw_name(
        scope,
        RESOURCE_MANAGER_GLOBAL_VAR_NAME,
        resource_container_object,
    );
}
