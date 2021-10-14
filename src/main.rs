extern crate pretty_env_logger;

mod service1;
mod system1;

use crate::service1::Service1;
use crate::system1::system1_untyped;
use fruity_any_derive::*;
use fruity_collections_derive::*;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_manager::EntityManager;
use fruity_ecs::initialize as initialize_ecs;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::world::World;
use fruity_ecs::*;
use fruity_ecs_derive::*;
use fruity_introspect_derive::*;
use fruity_javascript_scripting::error::log_js_error;
use fruity_javascript_scripting::initialize as initialize_javascript;
use fruity_javascript_scripting::runtime::JsRuntime;
use pretty_env_logger::formatted_builder;

#[derive(Debug, Clone, Component, Encodable, IntrospectFields, FruityAny)]
pub struct Component1 {
    pub float1: f64,
    // pub str1: String,
    pub int1: i32,
}

#[derive(Debug, Clone, Component, Encodable, IntrospectFields, FruityAny)]
pub struct Component2 {
    pub float1: f64,
}

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.try_init().unwrap();

    let world = World::new();
    initialize_ecs(&world);
    initialize_javascript(&world);

    let entity_manager = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<EntityManager>().unwrap()
    };

    let system_manager = {
        let service_manager = world.service_manager.read().unwrap();
        service_manager.get::<SystemManager>().unwrap()
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
        let service_manager = world.service_manager.read().unwrap();
        let js_runtime = service_manager.get::<JsRuntime>().unwrap();
        let mut js_runtime = js_runtime.write().unwrap();

        match js_runtime.run_module("src/javascript/index.js") {
            Ok(_) => (),
            Err(err) => log_js_error(&err),
        };
    }

    {
        let system_manager = system_manager.read().unwrap();
        system_manager.run();
        /*system_manager.run(&service_manager);
        system_manager.run(&service_manager);*/
    }
}
