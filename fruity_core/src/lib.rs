#![warn(missing_docs)]

//! ECS
//!
//! Provide an ECS, this ECS has hierarchy between all the entities and is intended to be easely extended by a scripting engine
//!
//! The ECS is organized with the following structure
//! - Resources are object that are shared all over the application, it can store services to provide function, intended to be used by the systems, for example a log service can provide functionalities to log things, everything is a service including the entity storage and the system storage
//! - Systems are function that do the logic part of the application, they can compute components and use resources
//! - Entities represent any object stored in the ecs, entities are composed of components, in a game engine, a game object for example
//! - Components are structure where the datas are stored

use crate::entity::entity_service::EntityService;
use crate::object_factory_service::ObjectFactoryService;
use crate::resource::resource_container::ResourceContainer;
use crate::system::system_service::SystemService;
use std::sync::Arc;

pub use fruity_core_derive::Component;
pub use fruity_core_derive::InstantiableObject;

#[macro_use]
extern crate lazy_static;

/// Tools to load dynamicaly modules
pub mod module;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// All related with resources
pub mod resource;

/// An observer pattern
pub mod signal;

/// Provides a collection for settings
pub mod settings;

/// Provides collection for systems
pub mod system;

/// Provides collection for a platform in wich the engine will be run
pub mod platform;

/// Provides some utils for the game engine
pub mod utils;

/// Provides a main object for the game engine
pub mod world;

/// Provides a factory for the introspect object
/// This will be used by the scripting language to expose object creation, especialy components
pub mod object_factory_service;

/// Create an entity, use it like entity![Box::new(component1), Box::new(component2)])
#[macro_export]
macro_rules! entity {
    ($($component:expr),*) => {
        fruity_core::entity::entity::Entity::new(vec![$ ($component),*])
    };
}

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($($component:expr),*) => {
        fruity_core::entity::entity::EntityTypeIdentifier(vec![$ ($component.to_string()),*])
    };
}

/// Initialize this extension
pub fn initialize(resource_container: Arc<ResourceContainer>) {
    //let module_manager = ModuleManager::new(resource_container.clone());
    let entity_service = EntityService::new(resource_container.clone());
    let system_service = SystemService::new(resource_container.clone());
    let object_factory_service = ObjectFactoryService::new(resource_container.clone());

    resource_container
        .add::<EntityService>("entity_service", Box::new(entity_service))
        .unwrap();
    resource_container
        .add::<SystemService>("system_service", Box::new(system_service))
        .unwrap();
    resource_container
        .add::<ObjectFactoryService>("object_factory_service", Box::new(object_factory_service))
        .unwrap();
}
