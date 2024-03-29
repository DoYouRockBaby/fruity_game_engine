use crate::components::camera::Camera;
use crate::components::rotate_2d::Rotate2d;
use crate::components::scale_2d::Scale2d;
use crate::components::sprite::Sprite;
use crate::components::transform_2d::Transform2d;
use crate::components::translate_2d::Translate2d;
use crate::graphic_2d_service::Graphic2dService;
use crate::systems::draw_camera::draw_camera;
use crate::systems::draw_sprite::draw_sprite;
use crate::systems::update_transform_2d::update_transform_2d;
use fruity_core::inject::Inject1;
use fruity_core::inject::Inject2;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;

pub mod components;
pub mod graphic_2d_service;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "graphic_2d_service";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let graphic_2d_service = Graphic2dService::new(resource_container.clone());

    resource_container.add::<Graphic2dService>("graphic_2d_service", Box::new(graphic_2d_service));

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Transform2d>("Transform2d");
    object_factory_service.register::<Translate2d>("Translate2d");
    object_factory_service.register::<Rotate2d>("Rotate2d");
    object_factory_service.register::<Scale2d>("Scale2d");
    object_factory_service.register::<Sprite>("Sprite");
    object_factory_service.register::<Camera>("Camera");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "update_transform_2d",
        MODULE_NAME,
        Inject1::new(update_transform_2d),
        SystemParams {
            pool_index: 95,
            ignore_pause: true,
        },
    );

    system_service.add_system(
        "draw_sprite",
        MODULE_NAME,
        Inject2::new(draw_sprite),
        SystemParams {
            pool_index: 98,
            ignore_pause: true,
        },
    );

    system_service.add_system(
        "draw_camera",
        MODULE_NAME,
        Inject2::new(draw_camera),
        SystemParams {
            pool_index: 99,
            ignore_pause: true,
        },
    );

    std::mem::drop(object_factory_service);
    std::mem::drop(system_service);
}
