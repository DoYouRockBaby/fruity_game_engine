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

/// Represent an entity composed by many components
#[macro_export]
macro_rules! entity {
    // `()` indicates that the macro takes no argument.
    ($($component:expr),*) => {
        vec![$ ($component),*] as Vec<&dyn fruity_ecs::component::component::Component>
    };
}
