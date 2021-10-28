use fruity_windows::windows_manager::WindowsManager;
use fruity_core::initialize as initialize_ecs;
use fruity_core::world::World;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_javascript_scripting::javascript_engine::JavascriptEngine;
use fruity_windows::initialize as initialize_windows;
use pretty_env_logger::formatted_builder;
use std::thread;

#[derive(Debug)]
pub struct WorldState {
    pub world: World,
}

impl Default for WorldState {
    fn default() -> Self {
        let mut builder = formatted_builder();
        builder.parse_filters("trace");
        builder.filter_module("iced_wgpu", log::LevelFilter::Off);
        builder.filter_module("naga", log::LevelFilter::Off);
        builder.filter_module("winit", log::LevelFilter::Off);
        builder.filter_module("mio", log::LevelFilter::Off);
        builder.filter_module("wgpu_core", log::LevelFilter::Off);
        builder.filter_module("wgpu_hal", log::LevelFilter::Off);
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
        javascript_engine.run_module("../assets/index.js");

        // Run the engine
        let service_manager = world.service_manager.clone();
        thread::spawn(move || {
            let service_manager = service_manager.read().unwrap();
            let windows_manager = service_manager.read::<WindowsManager>();
            windows_manager.run();
        });

        WorldState { world }
    }
}

#[derive(Debug, Clone)]
pub enum WorldMessage {}

pub fn update_world(_state: &mut WorldState, message: WorldMessage) {
    match message {}
}
