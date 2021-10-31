extern crate pretty_env_logger;

use fruity_core::initialize as initialize_core;
use fruity_core::module::module_manager::ModuleManager;
use fruity_core::world::World;
use pretty_env_logger::formatted_builder;

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.filter_module("naga", log::LevelFilter::Off);
    builder.filter_module("winit", log::LevelFilter::Off);
    builder.filter_module("mio", log::LevelFilter::Off);
    builder.filter_module("wgpu_core", log::LevelFilter::Off);
    builder.filter_module("wgpu_hal", log::LevelFilter::Off);
    builder.filter_module("iced_wgpu", log::LevelFilter::Off);
    builder.try_init().unwrap();

    let world = World::new();
    initialize_core(&world);

    // Load modules
    let module_manager = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<ModuleManager>().unwrap()
    };

    let module_manager = module_manager.read().unwrap();

    module_manager.load_module("./target/debug", "fruity_windows");
    module_manager.load_module("./target/debug", "fruity_graphic");
    module_manager.load_module("./target/debug", "fruity_graphic_2d");
    module_manager.load_module("./target/debug", "fruity_editor");
    module_manager.load_module("./target/debug", "fruity_javascript_scripting");
    std::mem::drop(module_manager);

    // Run the engine
    world.run();
}
