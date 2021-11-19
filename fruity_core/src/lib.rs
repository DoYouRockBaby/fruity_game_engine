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

use crate::object_factory_service::ObjectFactoryService;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::resource_reference::AnyResourceReference;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

/// Tools to load dynamicaly modules
pub mod module;

/// Traits to make types able to introspect themself
pub mod introspect;

/// Tools to implement data serialization
pub mod serialize;

/// All related with resources
pub mod resource;

/// Provides a tool to inject resources into functions
pub mod inject;

/// Traits similar to into and from but without some limitations
pub mod convert;

/// An observer pattern
pub mod signal;

/// Provides a collection for settings
pub mod settings;

/// Provides collection for a platform in wich the engine will be run
pub mod platform;

/// Provides some utils for the game engine
pub mod utils;

/// Provides a main object for the game engine
pub mod world;

/// Provides a factory for the introspect object
/// This will be used by the scripting language to expose object creation, especialy components
pub mod object_factory_service;

/// Initialize this extension
pub fn initialize(resource_container: Arc<ResourceContainer>) {
    //let module_manager = ModuleManager::new(resource_container.clone());
    let object_factory_service = ObjectFactoryService::new(resource_container.clone());

    resource_container
        .add::<ObjectFactoryService>("object_factory_service", Box::new(object_factory_service))
        .unwrap();

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<AnyResourceReference>("ResourceReference");
}
