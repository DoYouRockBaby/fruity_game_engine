#![warn(missing_docs)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - The world contain entities, services and systems
//! - Systems are function that do the logic part of the application, they can compute components and use services
//! - Services are object that can provide function, intended to be used by the systems, for example a log service can provide functionalities to log things
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// All related with services
pub mod service;

/// Provides collection for systems
pub mod system;

/// Provides collection for world
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
