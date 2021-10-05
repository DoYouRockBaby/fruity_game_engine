use rusty_v8 as v8;

use crate::module_specifier::resolve_path;
use crate::runtime::JsRuntime;
use fruity_ecs::world::world::World;
use std::convert::TryFrom;

mod error;
mod exception;
mod module_map;
mod module_specifier;
mod modules;
mod normalize_path;
mod runtime;
mod value;
mod value_implementations;

pub fn execute_script(_world: &mut World, path: &str) {
    // Initialize runtime
    let mut runtime = JsRuntime::new();
    runtime.set_func(
        "internprint",
        |scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue| {
            assert!(args.length() >= 2);

            assert!(args.get(0).is_function());
            assert!(args.get(1).is_function());

            let mut call_args = vec![];
            for i in 2..args.length() {
                call_args.push(args.get(i));
            }

            let receiver = args.this();
            let inspector_console_method =
                v8::Local::<v8::Function>::try_from(args.get(0)).unwrap();
            let deno_console_method = v8::Local::<v8::Function>::try_from(args.get(1)).unwrap();

            inspector_console_method.call(scope, receiver.into(), &call_args);
            deno_console_method.call(scope, receiver.into(), &call_args);
        },
    );

    // Try simple script running
    let result1 = runtime.run_script(r"3 + 4").unwrap().deserialize::<i32>();
    println!("Result: {:?}", result1);

    // Try module script running
    let specifier = resolve_path(path).unwrap();
    let module_id = runtime.load_main_module(&specifier).unwrap();
    let result2 = runtime.mod_evaluate(module_id);
    let result2 = result2.unwrap().deserialize::<i32>();

    println!("Result: {:?}", result2);
}
