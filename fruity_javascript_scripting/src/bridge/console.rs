extern crate pretty_env_logger;
use crate::JsRuntime;
use rusty_v8 as v8;

pub fn configure_console(runtime: &mut JsRuntime) {
    let global_object = runtime.global_object();
    let console_object = global_object.add_object("console");

    console_object.set_func(
        "log",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::debug!("{}", message));
        },
    );

    console_object.set_func(
        "debug",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::debug!("{}", message));
        },
    );

    console_object.set_func(
        "info",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::info!("{}", message));
        },
    );

    console_object.set_func(
        "warn",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::warn!("{}", message));
        },
    );

    console_object.set_func(
        "error",
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut _retval: v8::ReturnValue| {
            print_args(scope, args, |message| log::error!("{}", message));
        },
    );

    runtime.update_global_bindings();
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
