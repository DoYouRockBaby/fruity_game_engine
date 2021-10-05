use rusty_v8 as v8;

use crate::JsRuntime;
use std::convert::TryInto;

pub extern "C" fn host_initialize_import_meta_object_callback(
    context: v8::Local<v8::Context>,
    module: v8::Local<v8::Module>,
    meta: v8::Local<v8::Object>,
) {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };
    let module_map_rc = JsRuntime::module_map(scope);
    let module_map = module_map_rc.borrow();

    let module_global = v8::Global::new(scope, module);
    let info = module_map.get(&module_global).expect("Module not found");

    let url_key = v8::String::new(scope, "url").unwrap();
    let url_val = v8::String::new(scope, &info.filepath).unwrap();
    meta.create_data_property(scope, url_key.into(), url_val.into());
}

pub extern "C" fn host_import_module_dynamically_callback(
    context: v8::Local<v8::Context>,
    referrer: v8::Local<v8::ScriptOrModule>,
    specifier: v8::Local<v8::String>,
    _import_assertions: v8::Local<v8::FixedArray>,
) -> *mut v8::Promise {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };
    // NOTE(bartlomieju): will crash for non-UTF-8 specifier
    let specifier_str = specifier
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let referrer_name = referrer.get_resource_name();
    let referrer_name_str = referrer_name
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    // TODO(ry) I'm not sure what HostDefinedOptions is for or if we're ever going
    // to use it. For now we check that it is not used. This check may need to be
    // changed in the future.
    let host_defined_options = referrer.get_host_defined_options();
    assert_eq!(host_defined_options.length(), 0);
    let resolver = v8::PromiseResolver::new(scope).unwrap();
    let promise = resolver.get_promise(scope);
    let resolver_handle = v8::Global::new(scope, resolver);

    // Map errors from module resolution (not JS errors from module execution) to
    // ones rethrown from this scope, so they include the call stack of the
    // dynamic import site. Error objects without any stack frames are assumed to
    // be module resolution errors, other exception values are left as they are.
    let map_err =
        |scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue| {
            let arg = args.get(0);
            if arg.is_native_error() {
                let message = v8::Exception::create_message(scope, arg);
                if message.get_stack_trace(scope).unwrap().get_frame_count() == 0 {
                    let arg: v8::Local<v8::Object> = arg.try_into().unwrap();
                    let message_key = v8::String::new(scope, "message").unwrap();
                    let message = arg.get(scope, message_key.into()).unwrap();
                    let exception = v8::Exception::type_error(scope, message.try_into().unwrap());
                    scope.throw_exception(exception);
                    return;
                }
            }
            scope.throw_exception(arg);
        };
    let map_err = v8::FunctionTemplate::new(scope, map_err);
    let map_err = map_err.get_function(scope).unwrap();
    let promise = promise.catch(scope, map_err).unwrap();
    &*promise as *const _ as *mut _
}

pub extern "C" fn promise_reject_callback(message: v8::PromiseRejectMessage) {
    let scope = &mut unsafe { v8::CallbackScope::new(&message) };

    let state_rc = JsRuntime::state(scope);
    let mut state = state_rc.borrow_mut();

    let promise = message.get_promise();
    let promise_global = v8::Global::new(scope, promise);

    match message.get_event() {
        v8::PromiseRejectEvent::PromiseRejectWithNoHandler => {
            let error = message.get_value().unwrap();
            let error_global = v8::Global::new(scope, error);
        }
        v8::PromiseRejectEvent::PromiseHandlerAddedAfterReject => {}
        v8::PromiseRejectEvent::PromiseRejectAfterResolved => {}
        v8::PromiseRejectEvent::PromiseResolveAfterResolved => {
            // Should not warn. See #1272
        }
    };
}
