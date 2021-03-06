extern crate pretty_env_logger;

use fruity_core::settings::read_settings;
use fruity_core::settings::Settings;
use fruity_core::world::World;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::initialize as initialize_ecs;
use fruity_editor::initialize as initialize_editor;
use fruity_editor_graphic::initialize as initialize_editor_graphic;
use fruity_editor_graphic_2d::initialize as initialize_editor_graphic_2d;
use fruity_editor_hierarchy::initialize as initialize_editor_hierarchy;
use fruity_editor_javascript::initialize as initialize_editor_javascript;
use fruity_editor_physic_2d::initialize as initialize_editor_physic_2d;
use fruity_editor_physic_2d_rapier::initialize as initialize_editor_physic_2d_rapier;
use fruity_egui_editor::initialize as initialize_egui_editor;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_hierarchy::initialize as initialize_hierarchy;
use fruity_hierarchy_2d::initialize as initialize_hierarchy_2d;
use fruity_input::initialize as initialize_input;
use fruity_javascript::initialize as initialize_javascript;
use fruity_javascript_watcher::initialize as initialize_javascript_watcher;
use fruity_javascript_watcher::javascript_watcher_service::JavascriptWatcherService;
use fruity_physic_2d::initialize as initialize_physic_2d;
use fruity_physic_2d_rapier::initialize as initialize_physic_2d_rapier;
use fruity_wgpu_graphic::initialize as initialize_wgpu_graphic;
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
    initialize_ecs(world.resource_container.clone());
    world.run(
        |resource_container, settings| {
            initialize_hierarchy(resource_container.clone(), settings);
            initialize_winit_window(resource_container.clone(), settings);
            initialize_window(resource_container.clone(), settings);
            initialize_input(resource_container.clone(), settings);
            initialize_wgpu_graphic(resource_container.clone(), settings);
            initialize_winit_input(resource_container.clone(), settings);
            initialize_graphic(resource_container.clone(), settings);
            initialize_graphic_2d(resource_container.clone(), settings);
            initialize_physic_2d(resource_container.clone(), settings);
            initialize_physic_2d_rapier(resource_container.clone(), settings);
            initialize_hierarchy_2d(resource_container.clone(), settings);
            initialize_javascript(resource_container.clone(), settings);
            initialize_javascript_watcher(resource_container.clone(), settings);
            initialize_editor(resource_container.clone(), settings);
            initialize_egui_editor(resource_container.clone(), settings);
            initialize_editor_hierarchy(resource_container.clone(), settings);
            initialize_editor_graphic(resource_container.clone(), settings);
            initialize_editor_graphic_2d(resource_container.clone(), settings);
            initialize_editor_physic_2d(resource_container.clone(), settings);
            initialize_editor_javascript(resource_container.clone(), settings);
            initialize_editor_physic_2d_rapier(resource_container.clone(), settings);
        },
        |resource_container, settings| {
            // Load resources
            let resource_settings = settings.get::<Vec<Settings>>("resources", Vec::new());
            resource_container
                .clone()
                .load_resources_settings(resource_settings);

            // Load js script
            {
                let javascript_watcher_service =
                    resource_container.require::<JavascriptWatcherService>();
                let mut javascript_watcher_service = javascript_watcher_service.write();
                javascript_watcher_service.watch_module("./assets/index.js", "./assets");
            }
            /*resource_container
            .load_resource_file("./assets/index.js", "js")
            .unwrap();*/

            // Load entry scene
            {
                let entity_service = resource_container.require::<EntityService>();
                let entity_service = entity_service.read();
                entity_service.restore_from_file("./assets/scene.frsc");
            }

            /*let mut module_manager = ModuleManager::new(resource_container.clone());
            module_manager.load_module("./target/debug", "fruity_graphic");
            module_manager.load_module("./target/debug", "fruity_graphic_2d");
            module_manager.load_module("./target/debug", "fruity_editor");
            module_manager.load_module("./target/debug", "fruity_javascript");*/
        },
        &settings,
    );
}
