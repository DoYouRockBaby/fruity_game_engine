extern crate pretty_env_logger;

use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::settings::read_settings;
use fruity_core::settings::Settings;
use fruity_core::world::World;
use fruity_editor::initialize as initialize_editor;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_windows::platform;
use pretty_env_logger::formatted_builder;
use std::fs::File;
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

    let mut file = File::open("assets/settings.yaml").unwrap();
    let settings = read_settings(&mut file);

    let mut world = World::new();
    world.set_platform(platform);

    // Run the engine
    world.run(
        |service_manager, settings| {
            initialize_graphic(service_manager, settings);
            initialize_graphic_2d(service_manager, settings);
            initialize_editor(service_manager, settings);

            // Load resources
            {
                let service_manager = service_manager.read().unwrap();
                let mut resource_manager = service_manager.write::<ResourcesManager>();
                let resource_settings = settings.get::<Vec<Settings>>("resources", Vec::new());
                resource_manager.load_resources_settings(resource_settings);
            }

            initialize_javascript(service_manager, settings);

            /*let mut module_manager = ModuleManager::new(&service_manager);
            module_manager.load_module("./target/debug", "fruity_graphic");
            module_manager.load_module("./target/debug", "fruity_graphic_2d");
            module_manager.load_module("./target/debug", "fruity_editor");
            module_manager.load_module("./target/debug", "fruity_javascript_scripting");*/
        },
        &settings,
    );
}
