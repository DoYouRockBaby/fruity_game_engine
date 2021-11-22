use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use crate::math::vector2d::Vector2d;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod graphic_service;
pub mod math;
pub mod resources;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Vector2d>("Vector2d");
    object_factory_service.register::<Matrix3>("Matrix3");
    object_factory_service.register::<Matrix4>("Matrix4");
}
