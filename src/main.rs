extern crate pretty_env_logger;

use fruity_core::settings::read_settings;
use fruity_core::settings::Settings;
use fruity_core::world::World;
use fruity_editor::initialize as initialize_editor;
use fruity_editor_graphic_2d::initialize as initialize_editor_graphic_2d;
use fruity_editor_javascript::initialize as initialize_editor_javascript;
use fruity_egui_editor::initialize as initialize_egui_editor;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_input::initialize as initialize_input;
use fruity_javascript::initialize as initialize_javascript;
use fruity_wgpu_graphic::initialize as initialize_wgpu_graphic;
use fruity_wgpu_graphic_2d::initialize as initialize_wgpu_graphic_2d;
use fruity_windows::initialize as initialize_window;
use fruity_winit_input::initialize as initialize_winit_input;
use fruity_winit_windows::initialize as initialize_winit_window;
use fruity_winit_windows::platform;
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
    builder.try_init().unwrap();

    let mut file = File::open("assets/settings.yaml").unwrap();
    let settings = read_settings(&mut file);

    let mut world = World::new();
    world.set_platform(platform);

    // Run the engine
    world.run(
        |resource_manager, settings| {
            initialize_winit_window(resource_manager.clone(), settings);
            initialize_window(resource_manager.clone(), settings);
            initialize_input(resource_manager.clone(), settings);
            initialize_winit_input(resource_manager.clone(), settings);
            initialize_wgpu_graphic(resource_manager.clone(), settings);
            initialize_graphic(resource_manager.clone(), settings);
            initialize_wgpu_graphic_2d(resource_manager.clone(), settings);
            initialize_graphic_2d(resource_manager.clone(), settings);
            initialize_editor(resource_manager.clone(), settings);
            initialize_javascript(resource_manager.clone(), settings);
            initialize_egui_editor(resource_manager.clone(), settings);
            initialize_editor_graphic_2d(resource_manager.clone(), settings);
            initialize_editor_javascript(resource_manager.clone(), settings);

            // Load resources
            let resource_settings = settings.get::<Vec<Settings>>("resources", Vec::new());
            resource_manager
                .clone()
                .load_resources_settings(resource_settings);

            // Load js script
            resource_manager
                .load_resource_file("assets/index.js", "js")
                .unwrap();

            /*let mut module_manager = ModuleManager::new(&service_manager);
            module_manager.load_module("./target/debug", "fruity_graphic");
            module_manager.load_module("./target/debug", "fruity_graphic_2d");
            module_manager.load_module("./target/debug", "fruity_editor");
            module_manager.load_module("./target/debug", "fruity_javascript");*/
        },
        &settings,
    );
}
