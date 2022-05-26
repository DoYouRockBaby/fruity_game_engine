use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::serialize::serialize::serialize_v8;
use crate::JsObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::RwLock;
use rusty_v8 as v8;
use std::convert::TryFrom;
use std::sync::Arc;

impl JsObject {
    pub fn from_iterator(
        scope: &mut v8::HandleScope,
        iterator: Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>,
    ) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, "Iterator", iterator);
        object.set_func(scope, "next", iterator_next_callback, None);
        object.set_func(scope, "for_each", iterator_for_each_callback, None);

        object
    }
}

fn iterator_next_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this a as an iterator
    let intern_value = get_intern_value_from_v8_object::<
        Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>,
    >(scope, args.this());

    if let Some(iterator) = intern_value {
        // Call the function
        let mut iterator = iterator.write();
        let result = iterator.next();

        // Return the result
        //let serialized = serialize_v8(scope, &serialized);
        let result = match result {
            Some(value) => serialize_v8(scope, value),
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
    mut _return_value: v8::ReturnValue,
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
