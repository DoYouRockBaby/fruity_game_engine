extern crate pretty_env_logger;

mod service1;
mod system1;

use crate::service1::Service1;
use crate::system1::system1_untyped;
use fruity_any::*;
use fruity_ecs::component::components_factory::ComponentsFactory;
use fruity_ecs::entity::entity_manager::EntityManager;
use fruity_ecs::initialize as initialize_ecs;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourceLoaderParams;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::world::World;
use fruity_ecs::*;
use fruity_graphic::initialize as initialize_graphic;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;
use fruity_graphic_2d::components::sprite::Sprite;
use fruity_graphic_2d::initialize as initialize_graphic_2d;
use fruity_introspect::*;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_javascript_scripting::javascript_engine::JavascriptEngine;
use fruity_windows::initialize as initialize_windows;
use fruity_windows::windows_manager::WindowsManager;
use pretty_env_logger::formatted_builder;
use std::fs::File;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Component1 {
    pub float1: f64,
    // pub str1: String,
    pub int1: i32,
}

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Component2 {
    pub float1: f64,
}

pub fn init_system(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();

    // Load resources
    let mut resources_manager = service_manager.write::<ResourcesManager>();

    let settings_path = "assets/resources.yaml";
    let mut settings_file = File::open(settings_path).unwrap();
    resources_manager
        .load_resource(
            ResourceIdentifier(settings_path.to_string()),
            "resource_settings",
            &mut settings_file,
            ResourceLoaderParams::new(),
        )
        .unwrap();

    // Create components
    let mut entity_manager = service_manager.write::<EntityManager>();

    entity_manager.create(entity!(
        Box::new(Position { x: 0.25, y: 0.25 }),
        Box::new(Size {
            width: 0.5,
            height: 0.5,
        }),
        Box::new(Sprite {
            texture: resources_manager
                .get_resource(ResourceIdentifier("assets/logo.png".to_string()))
        })
    ));
}

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.try_init().unwrap();

    let world = World::new();
    initialize_ecs(&world);
    initialize_windows(&world);
    initialize_graphic(&world);
    initialize_graphic_2d(&world);

    // Initialize custom component
    {
        let service_manager = world.service_manager.read().unwrap();
        let mut components_factory = service_manager.write::<ComponentsFactory>();

        components_factory.add("Component1", || {
            Box::new(Component1 {
                float1: 0.0,
                int1: 0,
            })
        });

        components_factory.add("Component2", || Box::new(Component2 { float1: 0.0 }));
    }

    // Initialize custom services
    {
        let mut service_manager = world.service_manager.write().unwrap();
        service_manager.register::<Service1>("service1", Service1::new());
    }

    // Initialized custom systems
    {
        let service_manager = world.service_manager.read().unwrap();
        let mut system_manager = service_manager.write::<SystemManager>();
        system_manager.add_system(system1_untyped);
        system_manager.add_begin_system(init_system);
    }

    initialize_javascript(&world);

    // Run the javascript module
    {
        let service_manager = world.service_manager.read().unwrap();
        let javascript_engine = service_manager.write::<JavascriptEngine>();
        javascript_engine.run_module("src/javascript/index.js");
    }

    // Run the engine
    {
        let service_manager = world.service_manager.read().unwrap();
        let windows_manager = service_manager.read::<WindowsManager>();
        windows_manager.run();
    }
}
