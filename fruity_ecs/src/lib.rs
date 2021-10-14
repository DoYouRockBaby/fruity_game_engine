#![warn(missing_docs)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Services are object that can provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use services
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use crate::entity::entity_manager::EntityManager;
use crate::service::service_manager::ServiceManager;
use crate::system::system_manager::SystemManager;
use crate::world::World;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// Provides a collection for services
pub mod service;

/// Provides structure to pass object between the rust ecosystem and the scripting system
pub mod serialize;

/// Provides collection for systems
pub mod system;

/// Provides a main object for the game engine
pub mod world;

/// Create an entity, use it like entity![Box::new(component1), Box::new(component2)])
#[macro_export]
macro_rules! entity {
    ($($component:expr),*) => {
        fruity_ecs::entity::entity::Entity::new(vec![$ ($component),*])
    };
}

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($($component:expr),*) => {
        fruity_ecs::entity::entity::EntityTypeIdentifier(vec![$ ($component.to_string()),*])
    };
}

/// Initialize this extension
pub fn initialize(world: &World) {
    let mut service_manager = world.service_manager.write().unwrap();
    service_manager.register("entity_manager", EntityManager::new(world));
    service_manager.register("system_manager", SystemManager::new(world));
}
