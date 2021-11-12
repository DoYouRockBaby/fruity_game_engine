use crate::components::camera::Camera;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphic_2d_manager::Graphic2dManager;
use crate::math::vector2d::Vector2d;
use crate::resources::default_resources::load_default_resources;
use crate::systems::draw_camera::draw_camera_untyped;
use crate::systems::draw_sprite::draw_sprite_untyped;
use fruity_core::object_factory::ObjectFactory;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;

pub mod components;
pub mod graphic_2d_manager;
pub mod math;
pub mod resources;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let object_factory = resource_manager.require::<ObjectFactory>("object_factory");
    let mut object_factory = object_factory.write();

    object_factory.register::<Position>("Position");
    object_factory.register::<Size>("Size");
    object_factory.register::<Sprite>("Sprite");
    object_factory.register::<Camera>("Camera");
    object_factory.register::<Vector2d>("Vector2d");

    let system_manager = resource_manager.require::<SystemManager>("system_manager");
    let mut system_manager = system_manager.write();

    system_manager.add_system_that_ignore_pause(draw_camera_untyped, Some(97));
    system_manager.add_system_that_ignore_pause(draw_sprite_untyped, Some(98));

    std::mem::drop(object_factory);
    std::mem::drop(system_manager);

    load_default_resources(resource_manager);
}
