use crate::js_value::object::JsObject;
use crate::js_value::object::JsObjectInternalObject;
use crate::serialize::serialize::serialize_v8;
use crate::JsRuntime;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::world::World;
use rusty_v8 as v8;

pub fn configure_services(runtime: &mut JsRuntime, world: &World) {
    let global_object = runtime.global_object();

    let mut service_manager_js = JsObject::from_internal(JsObjectInternalObject::ServiceManager(
        world.service_manager.clone(),
    ));

    service_manager_js.set_func(
        "get",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut return_value: v8::ReturnValue| {
            // Get this as service
            let this = args.this().get_internal_field(scope, 0).unwrap();
            let this = unsafe { v8::Local::<v8::External>::cast(this) };
            let this = this.value() as *const JsObjectInternalObject;
            let this = unsafe { this.as_ref().unwrap().clone() };

            if let JsObjectInternalObject::ServiceManager(this) = this {
                // Build the arguments
                let name = args
                    .get(0)
                    .to_string(scope)
                    .unwrap()
                    .to_rust_string_lossy(scope);

                // Call the function
                let reader = this.read().unwrap();
                let this = &*reader;
                let result = this.get_by_name(&name);

                // Return the result
                if let Some(result) = result {
                    let serialized = serialize_v8(scope, &Serialized::Service(result));

                    if let Some(serialized) = serialized {
                        return_value.set(serialized.into());
                    }
                }
            }
        },
    );

    global_object.add_field("services", service_manager_js);
}
