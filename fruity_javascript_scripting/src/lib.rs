use rusty_v8 as v8;

use crate::runtime::JsRuntime;
use fruity_ecs::world::world::World;

mod error;
mod exception;
mod module_map;
mod normalize_path;
mod runtime;
mod value;
mod value_implementations;

pub fn execute_script(_world: &mut World, path: &str) {
    // Initialize runtime
    let mut runtime = JsRuntime::new();
    runtime.set_func("log", log);

    // Try simple script running
    let result1 = runtime
        .run_script(r"log('test'); 3 + 4")
        .unwrap()
        .deserialize::<i32>();
    println!("Result: {:#?}", result1);

    // Try module script running
    let result2 = runtime.run_module(path).unwrap().deserialize::<i32>();
    println!("Result: {:#?}", result2);
}

fn log(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _retval: v8::ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);

    println!("Logged: {}", message);
}
