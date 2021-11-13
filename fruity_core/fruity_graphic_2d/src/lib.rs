use crate::components::camera::Camera;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphic_2d_service::Graphic2dService;
use crate::math::vector2d::Vector2d;
use crate::resources::default_resources::load_default_resources;
use crate::systems::draw_camera::draw_camera_untyped;
use crate::systems::draw_sprite::draw_sprite_untyped;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod components;
pub mod graphic_2d_service;
pub mod math;
pub mod resources;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let object_factory_service =
        resource_container.require::<ObjectFactoryService>("object_factory_service");
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Position>("Position");
    object_factory_service.register::<Size>("Size");
    object_factory_service.register::<Sprite>("Sprite");
    object_factory_service.register::<Camera>("Camera");
    object_factory_service.register::<Vector2d>("Vector2d");

    let system_service = resource_container.require::<SystemService>("system_service");
    let mut system_service = system_service.write();

    system_service.add_system_that_ignore_pause(draw_camera_untyped, Some(97));
    system_service.add_system_that_ignore_pause(draw_sprite_untyped, Some(98));

    std::mem::drop(object_factory_service);
    std::mem::drop(system_service);

    load_default_resources(resource_container);
}
