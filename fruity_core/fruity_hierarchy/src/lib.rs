use crate::components::camera::Camera;
use crate::components::parent::Parent;
use crate::components::position::Position;
use crate::components::size::Size;
use crate::components::sprite::Sprite;
use crate::graphic_2d_service::Graphic2dService;
use crate::math::vector2d::Vector2d;
use crate::resources::default_resources::load_default_resources;
use crate::systems::delete_cascade::delete_cascade;
use crate::systems::draw_camera::draw_camera;
use crate::systems::draw_sprite::draw_sprite;
use fruity_core::inject::Inject1;
use fruity_core::inject::Inject2;
use fruity_core::inject::Inject3;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod components;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Parent>("Parent");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_begin_system(Inject1::new(delete_cascade), None);
}
