extern crate pretty_env_logger;

use fruity_core::world::World;
use fruity_editor::initialize as initialize_editor;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_windows::platform;
use pretty_env_logger::formatted_builder;
//use fruity_core::module::module_manager::ModuleManager;

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

    let mut world = World::new();
    world.set_platform(platform);

    // Run the engine
    world.run(|service_manager| {
        initialize_graphic(service_manager);
        initialize_graphic_2d(service_manager);
        initialize_editor(service_manager);
        initialize_javascript(service_manager);

        /*let mut module_manager = ModuleManager::new(&service_manager);
        module_manager.load_module("./target/debug", "fruity_graphic");
        module_manager.load_module("./target/debug", "fruity_graphic_2d");
        module_manager.load_module("./target/debug", "fruity_editor");
        module_manager.load_module("./target/debug", "fruity_javascript_scripting");*/
    });
}
