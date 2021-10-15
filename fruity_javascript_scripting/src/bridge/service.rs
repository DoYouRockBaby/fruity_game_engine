use crate::js_value::object::JsObject;
use crate::JsRuntime;
use fruity_ecs::world::World;

pub fn configure_services(runtime: &mut JsRuntime, world: &World) {
    let mut handles = runtime.handles.lock().unwrap();
    let mut global_object = handles.global_object();
    let scope = &mut handles.handle_scope();

    let service_manager_object =
        JsObject::from_service_manager(scope, world.service_manager.clone());

    global_object.add_field(scope, "services", service_manager_object);
}
