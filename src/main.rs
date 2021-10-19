extern crate pretty_env_logger;

use fruity_ecs::initialize as initialize_ecs;
use fruity_ecs::world::World;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_javascript_scripting::javascript_engine::JavascriptEngine;
use fruity_windows::initialize as initialize_windows;
use fruity_windows::windows_manager::WindowsManager;
use pretty_env_logger::formatted_builder;

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.try_init().unwrap();

    let world = World::new();
    initialize_ecs(&world);
    initialize_windows(&world);
    initialize_graphic(&world);
    initialize_graphic_2d(&world);
    initialize_javascript(&world);

    // Run the javascript module
    let javascript_engine = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<JavascriptEngine>().unwrap()
    };

    let javascript_engine = javascript_engine.read().unwrap();
    javascript_engine.run_module("assets/index.js");

    // Run the engine
    {
        let service_manager = world.service_manager.read().unwrap();
        let windows_manager = service_manager.read::<WindowsManager>();
        windows_manager.run();
    }
}
