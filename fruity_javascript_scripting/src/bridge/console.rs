extern crate pretty_env_logger;
use crate::js_value::object::JsObject;
use crate::JsRuntime;
use rusty_v8 as v8;

pub fn configure_console(runtime: &mut JsRuntime) {
    let mut handles = runtime.handles.lock().unwrap();
    let mut global_object = handles.global_object();
    let scope = &mut handles.handle_scope();
    let mut console_object = JsObject::new(scope);

    console_object.set_func(
        scope,
        "log",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::debug!("{}", message));
        },
        None,
    );

    console_object.set_func(
        scope,
        "debug",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::debug!("{}", message));
        },
        None,
    );

    console_object.set_func(
        scope,
        "info",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::info!("{}", message));
        },
        None,
    );

    console_object.set_func(
        scope,
        "warn",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::warn!("{}", message));
        },
        None,
    );

    console_object.set_func(
        scope,
        "error",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::error!("{}", message));
        },
        None,
    );

    global_object.add_field(scope, "console", console_object);
}

fn print_args<F: Fn(&str)>(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    print: F,
) {
    let mut message = String::new();
    for i in 0..args.length() {
        let arg_str = &args
            .get(i)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        if i == 0 {
            message += &arg_str;
        } else {
            message += " ";
            message += &arg_str;
        }
    }

    print(&message);
}
