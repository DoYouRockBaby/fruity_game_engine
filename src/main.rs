extern crate pretty_env_logger;

use fruity_core::initialize as initialize_ecs;
use fruity_core::module::load_modules;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::world::World;
use pretty_env_logger::formatted_builder;
use std::fs::File;

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
    initialize_ecs(&world);
    load_modules(&world, "./").unwrap();
    /*initialize_windows(&world);
    initialize_graphic(&world);
    initialize_graphic_2d(&world);
    initialize_javascript(&world);
    initialize_editor(&world);*/

    // Run the javascript main module
    {
        let service_manager = world.service_manager.read().unwrap();
        let mut resources_manager = service_manager.write::<ResourcesManager>();
        resources_manager
            .load_resource_file("assets/index.js", "js")
            .unwrap();
    };

    // Run the engine
    {
        //run
    }
}
