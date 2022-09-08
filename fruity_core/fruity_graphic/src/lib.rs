use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use crate::math::vector2d::Vector2d;
use crate::math::Color;
use crate::resources::default_resources::load_default_resources;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use crate::resources::texture_resource::load_texture;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;

pub mod graphic_service;
pub mod math;
pub mod resources;

/// The module name
pub static MODULE_NAME: &str = "graphic_service";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Color>("Color");
    object_factory_service.register::<Vector2d>("Vector2d");
    object_factory_service.register::<Matrix3>("Matrix3");
    object_factory_service.register::<Matrix4>("Matrix4");

    resource_container.add_resource_loader("material", load_material);
    resource_container.add_resource_loader("wgsl", load_shader);
    resource_container.add_resource_loader("material", load_material);
    resource_container.add_resource_loader("png", load_texture);
    resource_container.add_resource_loader("jpeg", load_texture);
    resource_container.add_resource_loader("jpg", load_texture);
    resource_container.add_resource_loader("gif", load_texture);
    resource_container.add_resource_loader("bmp", load_texture);
    resource_container.add_resource_loader("ico", load_texture);
    resource_container.add_resource_loader("tiff", load_texture);

    load_default_resources(resource_container);
}
