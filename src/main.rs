extern crate pretty_env_logger;

mod service1;
mod system1;

use crate::service1::Service1;
use crate::system1::system1_untyped;
use fruity_any::*;
use fruity_ecs::component::components_factory::ComponentsFactory;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_manager::EntityManager;
use fruity_ecs::initialize as initialize_ecs;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::world::World;
use fruity_ecs::*;
use fruity_graphic::initialize as fruity_graphic;
use fruity_introspect::*;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_javascript_scripting::javascript_engine::JavascriptEngine;
use fruity_windows::initialize as initialize_windows;
use fruity_windows::windows_manager::WindowsManager;
use pretty_env_logger::formatted_builder;
use std::fs::File;

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

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.try_init().unwrap();

    let world = World::new();
    initialize_ecs(&world);
    initialize_windows(&world);
    fruity_graphic(&world);

    // Initialize resources
    {
        let service_manager = world.service_manager.read().unwrap();
        let resources_manager = service_manager.get::<ResourcesManager>().unwrap();
        let mut resources_manager = resources_manager.write().unwrap();

        let settings_path = "assets/resources.yaml";
        let mut settings_file = File::open(settings_path).unwrap();
        resources_manager
            .load_resource(
                ResourceIdentifier(settings_path.to_string()),
                "resource_settings",
                &mut settings_file,
            )
            .unwrap();
    }

    // Initialize component
    {
        let service_manager = world.service_manager.read().unwrap();
        let components_factory = service_manager.get::<ComponentsFactory>().unwrap();
        let mut components_factory = components_factory.write().unwrap();

        components_factory.add("Component1", || {
            Box::new(Component1 {
                float1: 0.0,
                int1: 0,
            })
        });

        components_factory.add("Component2", || Box::new(Component2 { float1: 0.0 }));
    }

    initialize_javascript(&world);

    let entity_manager = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<EntityManager>().unwrap()
    };

    let system_manager = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<SystemManager>().unwrap()
    };

    let javascript_engine = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<JavascriptEngine>().unwrap()
    };

    {
        let mut entity_manager = entity_manager.write().unwrap();
        let mut system_manager = system_manager.write().unwrap();

        let component1 = Component1 {
            float1: 3.14,
            // str1: "je suis une string 1".to_string(),
            int1: 12,
        };

        let component2 = Component2 { float1: 3.14 };

        let component3 = Component1 {
            float1: 3.14,
            // str1: "je suis une string 2".to_string(),
            int1: 34,
        };
        let component4 = Component1 {
            float1: 3.14,
            // str1: "je suis une string 3".to_string(),
            int1: 53,
        };

        let component5 = Component2 { float1: 2.14 };
        let component6 = Component1 {
            float1: 3.14,
            // str1: "je suis une string 4".to_string(),
            int1: 43,
        };

        let component7 = Component2 { float1: 5.14 };

        let _entity_id_1 =
            entity_manager.create(entity!(Box::new(component1), Box::new(component2)));
        let _entity_id_2 = entity_manager.create(entity!(Box::new(component3)));
        let entity_id_3 =
            entity_manager.create(entity!(Box::new(component4), Box::new(component5)));
        let entity_id_4 =
            entity_manager.create(entity!(Box::new(component6), Box::new(component7)));

        entity_manager.remove(entity_id_3);
        entity_manager.remove(EntityId(0));

        let mut service_manager = world.service_manager.write().unwrap();
        service_manager.register::<Service1>("service1", Service1::new());
        system_manager.add_system(system1_untyped);

        // println!("{:#?}", world);
        println!("{:#?}", entity_manager.get(entity_id_4));
    }

    {
        // Javascript test
        let javascript_engine = javascript_engine.read().unwrap();
        javascript_engine.run_module("src/javascript/index.js");
    }

    {
        let windows_manager = {
            let service_manager = world.service_manager.read().unwrap();
            service_manager.get::<WindowsManager>().unwrap()
        };

        let windows_manager = windows_manager.read().unwrap();
        windows_manager.run();

        /*let system_manager = system_manager.read().unwrap();
        system_manager.run();
        system_manager.run();
        system_manager.run(&service_manager);*/
    }
}
