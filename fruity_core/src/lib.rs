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
use crate::object_factory::ObjectFactory;
use crate::resource::load_resources::load_resources;
use crate::resource::resources_manager::ResourcesManager;
use crate::service::service_manager::ServiceManager;
use crate::system::system_manager::SystemManager;
use std::sync::Arc;
use std::sync::RwLock;

pub use fruity_core_derive::Component;

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

/// Provides a collection for services
pub mod service;

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
pub mod object_factory;

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

/// A callback that is used to run the game engine
pub type RunCallback = Box<dyn FnOnce()>;

/// Initialize this extension
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>) -> Option<RunCallback> {
    //let module_manager = ModuleManager::new(service_manager);
    let entity_manager = EntityManager::new(service_manager);
    let system_manager = SystemManager::new(service_manager);
    let object_factory = ObjectFactory::new(service_manager);
    let resources_manager = ResourcesManager::new(service_manager);

    let mut service_manager = service_manager.write().unwrap();
    service_manager.register("entity_manager", entity_manager);
    service_manager.register("system_manager", system_manager);
    service_manager.register("object_factory", object_factory);
    service_manager.register("resources_manager", resources_manager);

    let mut resources_manager = service_manager.write::<ResourcesManager>();
    resources_manager.add_resource_loader("resource_settings", load_resources);

    None
}
