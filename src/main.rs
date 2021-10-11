extern crate pretty_env_logger;

mod service1;
mod system1;

use crate::service1::Service1;
use crate::system1::system1_untyped;
use fruity_any_derive::*;
use fruity_collections_derive::*;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::world::world::World;
use fruity_ecs::*;
use fruity_ecs_derive::*;
use fruity_introspect_derive::*;
use fruity_javascript_scripting::execute_script;
use pretty_env_logger::formatted_builder;

#[derive(Debug, Clone, Component, Encodable, IntrospectFields, FruityAny)]
pub struct Component1 {
    pub float1: f64,
    pub str1: String,
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

    let mut world = World::new();
    let component1 = Component1 {
        float1: 3.14,
        str1: "je suis une string 1".to_string(),
        int1: 12,
    };

    let component2 = Component2 { float1: 3.14 };

    let component3 = Component1 {
        float1: 3.14,
        str1: "je suis une string 2".to_string(),
        int1: 34,
    };
    let component4 = Component1 {
        float1: 3.14,
        str1: "je suis une string 3".to_string(),
        int1: 53,
    };

    let component5 = Component2 { float1: 2.14 };
    let component6 = Component1 {
        str1: "je suis une string 4".to_string(),
        float1: 3.14,
        int1: 43,
    };

    let component7 = Component2 { float1: 5.14 };

    let entity_id_1 = world
        .entity_manager
        .create(entity!(Box::new(component1), Box::new(component2)));
    let entity_id_2 = world.entity_manager.create(entity!(Box::new(component3)));
    let entity_id_3 = world
        .entity_manager
        .create(entity!(Box::new(component4), Box::new(component5)));
    let entity_id_4 = world
        .entity_manager
        .create(entity!(Box::new(component6), Box::new(component7)));

    world.entity_manager.remove(entity_id_3);
    world.entity_manager.remove(EntityId(0));

    match world.entity_manager.get(entity_id_1) {
        Some(entity) => match entity.write().unwrap().get_mut(1) {
            Some(component) => component.set_field("float1", 5432.1 as f64),
            None => (),
        },
        None => (),
    }

    match world.entity_manager.get(entity_id_2) {
        Some(entity) => match entity.write().unwrap().get_mut(0) {
            Some(component) => component.set_field("int1", 12345 as i64),
            None => (),
        },
        None => (),
    }

    world.service_manager.register::<Service1>(Service1::new());
    world.system_manager.add_system(system1_untyped);

    let script_path = "src/javascript/index.js";
    execute_script(&mut world, script_path);

    // println!("{:#?}", world);
    /*println!("{:#?}", world.entity_manager.get(entity_id_4));

    world.run();
    world.run();
    world.run();*/
}
