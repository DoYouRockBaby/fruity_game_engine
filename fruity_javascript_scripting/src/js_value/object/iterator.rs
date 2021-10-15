use crate::js_value::utils::get_intern_value_from_v8_args;
use crate::runtime::JsRuntimeHandles;
use crate::serialize::serialize::serialize_v8;
use crate::JsObject;
use fruity_ecs::serialize::serialized::Serialized;
use rusty_v8 as v8;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

impl JsObject {
    pub fn from_iterator(
        handles: Arc<Mutex<JsRuntimeHandles>>,
        iterator: Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>,
    ) -> JsObject {
        let object = JsObject::from_intern_value(handles, iterator);
        object.set_func(handles, "next", iterator_next_callback);
        object.set_func(handles, "for_each", iterator_for_each_callback);

        object
    }
}

fn iterator_next_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this an entity
    let intern_value = get_intern_value_from_v8_args::<
        Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>,
    >(scope, &args);

    if let Some(iterator) = intern_value {
        // Call the function
        let mut iterator = iterator.write().unwrap();
        let result = iterator.next();

        // Return the result
        //let serialized = serialize_v8(scope, &serialized);
        let result = match result {
            Some(value) => serialize_v8(scope, &value),
            None => None,
        };

        let return_object = match result {
            Some(value) => {
                let result = v8::Object::new(scope);

                // Set done to false
                let done = v8::Boolean::new(scope, false);
                let done_string = v8::String::new(scope, "done").unwrap();
                result.set(scope, done_string.into(), done.into());

                // Set value
                let value_string = v8::String::new(scope, "value").unwrap();
                result.set(scope, value_string.into(), value);

                result
            }
            None => {
                let result = v8::Object::new(scope);

                // Set done to true
                let done = v8::Boolean::new(scope, true);
                let done_string = v8::String::new(scope, "done").unwrap();
                result.set(scope, done_string.into(), done.into());

                result
            }
        };

        return_value.set(return_object.into());
    }
}

fn iterator_for_each_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    let this = args.this();

    // Get callback
    let callback = args.get(0);
    match v8::Local::<v8::Function>::try_from(callback) {
        Ok(callback) => {
            // Get next function
            let next_string = v8::String::new(scope, "next").unwrap();
            let next = this.get(scope, next_string.into()).unwrap();
            let next = v8::Local::<v8::Function>::try_from(next).unwrap();

            // Execute next function
            let next_value = next.call(scope, this.into(), &[]).unwrap();
            let mut next_value_object = v8::Local::<v8::Object>::try_from(next_value).unwrap();

            // Get the next result.done value
            let done_string = v8::String::new(scope, "done").unwrap();
            let mut next_done_value = next_value_object
                .get(scope, done_string.into())
                .unwrap()
                .boolean_value(scope);

            // Iterate for each element
            while !next_done_value {
                // Get the current iterator value
                let value_string = v8::String::new(scope, "value").unwrap();
                let next_value_value = next_value_object.get(scope, value_string.into()).unwrap();

                // Call the callback
                let undefined = v8::undefined(scope);
                callback.call(scope, undefined.into(), &[next_value_value]);

                // Get the next result.done value
                match next.call(scope, this.into(), &[]) {
                    Some(next_value) => {
                        next_value_object = v8::Local::<v8::Object>::try_from(next_value).unwrap();
                        next_done_value = next_value_object
                            .get(scope, done_string.into())
                            .unwrap()
                            .boolean_value(scope);
                    }
                    None => {
                        next_done_value = true;
                    }
                }
            }
        }
        Err(_) => (),
    };
}
