use crate::components::camera::Camera;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphics_2d_manager::Graphics2dManager;
use crate::systems::draw_camera::draw_camera_untyped;
use crate::systems::draw_sprite::draw_sprite_untyped;
use fruity_core::object_factory::ObjectFactory;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;
use std::sync::RwLock;

pub mod components;
pub mod graphics_2d_manager;
pub mod systems;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let graphic_2d_manager = Graphics2dManager::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("graphic_2d_manager", graphic_2d_manager);

    let mut object_factory = service_manager_writer.write::<ObjectFactory>();

    object_factory.register::<Position>("Position");
    object_factory.register::<Size>("Size");
    object_factory.register::<Sprite>("Sprite");
    object_factory.register::<Camera>("Camera");

    let mut system_manager = service_manager_writer.write::<SystemManager>();
    system_manager.add_system(draw_camera_untyped, Some(97));
    system_manager.add_system(draw_sprite_untyped, Some(98));
}
