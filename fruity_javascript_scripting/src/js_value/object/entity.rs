use crate::js_value::utils::get_intern_value_from_v8_args;
use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::JsObject;
use fruity_ecs::entity::entity_rwlock::EntityRwLock;
use fruity_ecs::serialize::serialized::Serialized;
use rusty_v8 as v8;

impl JsObject {
    pub fn from_entity(scope: &mut v8::HandleScope, entity: EntityRwLock) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, entity);
        object.set_func(scope, "length", entity_length_callback);

        object
    }
}

fn entity_length_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as an entity
    let intern_value = get_intern_value_from_v8_args::<EntityRwLock>(scope, &args);

    if let Some(entity) = intern_value {
        // Call the function
        let entity = entity.read().unwrap();
        let result = entity.len();

        // Return the result
        inject_serialized_into_v8_return_value(
            scope,
            &Serialized::USize(result),
            &mut return_value,
        );
    }
}
