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
use crate::system::system_service::BeginSystemParams;
use crate::system::system_service::EndSystemParams;
use crate::system::system_service::SystemParams;
use crate::system::system_service::SystemService;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use std::sync::Arc;

pub use fruity_ecs_derive::Component;
pub use fruity_ecs_derive::InstantiableObject;
pub use fruity_ecs_derive::IntrospectObject;

/// All related with components
pub mod component;

/// All related with entities
pub mod entity;

/// Provides collection for systems
pub mod system;

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($($component:expr),*) => {
        fruity_ecs::entity::entity::EntityTypeIdentifier(vec![$ ($component.to_string()),*])
    };
}

/// The module name
pub static MODULE_NAME: &str = "fruity_ecs";

/// Initialize this extension
// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>) {
    let entity_service = EntityService::new(resource_container.clone());
    let system_service = SystemService::new(resource_container.clone());

    resource_container.add::<EntityService>("entity_service", Box::new(entity_service));
    resource_container.add::<SystemService>("system_service", Box::new(system_service));

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<SystemParams>("SystemParams");
    object_factory_service.register::<BeginSystemParams>("BeginSystemParams");
    object_factory_service.register::<EndSystemParams>("EndSystemParams");
}
