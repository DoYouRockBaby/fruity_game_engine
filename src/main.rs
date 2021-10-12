extern crate pretty_env_logger;

mod service1;
mod system1;

use crate::service1::Service1;
use crate::system1::system1_untyped;
use fruity_any_derive::*;
use fruity_collections_derive::*;
use fruity_core::service_manager::ServiceManager;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_manager::EntityManager;
use fruity_ecs::initialize as initialize_ecs;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_ecs::*;
use fruity_ecs_derive::*;
use fruity_introspect_derive::*;
use fruity_javascript_scripting::execute_script;
use pretty_env_logger::formatted_builder;

#[derive(Debug, Clone, Component, Encodable, IntrospectFields, FruityAny)]
pub struct Component1 {
    pub float1: f64,
    // pub str1: String,
    pub int1: i64,
}

#[derive(Debug, Clone, Component, Encodable, IntrospectFields, FruityAny)]
pub struct Component2 {
    pub float1: f64,
}

fn main() {
    let mut builder = formatted_builder();
    builder.parse_filters("trace");
    builder.try_init().unwrap();

    let mut service_manager = ServiceManager::new();
    initialize_ecs(&mut service_manager);

    let entity_manager = service_manager.get::<EntityManager>().unwrap();
    let system_manager = service_manager.get::<SystemManager>().unwrap();

    {
        let mut entity_manager_writer = entity_manager.write().unwrap();
        let mut system_manager_writer = system_manager.write().unwrap();

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

        let entity_id_1 =
            entity_manager_writer.create(entity!(Box::new(component1), Box::new(component2)));
        let entity_id_2 = entity_manager_writer.create(entity!(Box::new(component3)));
        let entity_id_3 =
            entity_manager_writer.create(entity!(Box::new(component4), Box::new(component5)));
        let entity_id_4 =
            entity_manager_writer.create(entity!(Box::new(component6), Box::new(component7)));

        entity_manager_writer.remove(entity_id_3);
        entity_manager_writer.remove(EntityId(0));

        match entity_manager_writer.get(entity_id_1) {
            Some(entity) => match entity.write().unwrap().get_mut(1) {
                Some(component) => component.set_field("float1", 5432.1 as f64),
                None => (),
            },
            None => (),
        }

        match entity_manager_writer.get(entity_id_2) {
            Some(entity) => match entity.write().unwrap().get_mut(0) {
                Some(component) => component.set_field("int1", 12345 as i64),
                None => (),
            },
            None => (),
        }

        service_manager.register::<Service1>(Service1::new());
        // system_manager_writer.add_system(system1_untyped);

        // println!("{:#?}", world);
        println!("{:#?}", entity_manager_writer.get(entity_id_4));
    }

    let script_path = "src/javascript/index.js";
    execute_script(&mut service_manager, script_path);

    {
        let system_manager_reader = system_manager.read().unwrap();

        system_manager_reader.run(&mut service_manager);
        system_manager_reader.run(&mut service_manager);
        system_manager_reader.run(&mut service_manager);
    }
}
