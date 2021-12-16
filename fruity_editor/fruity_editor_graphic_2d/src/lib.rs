use crate::gizmos_service::GizmosService;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d;
use fruity_core::inject::Inject1;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;
use fruity_editor::component_editor_service::ComponentEditorService;
use fruity_editor::component_editor_service::RegisterComponentParams;
use std::sync::Arc;

pub mod gizmos_service;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_graphic_2d";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let gizmos_service = GizmosService::new(resource_container.clone());

    resource_container.add::<GizmosService>("gizmos_service", Box::new(gizmos_service));

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "draw_gizmos_2d",
        MODULE_NAME,
        Inject1::new(draw_gizmos_2d),
        Some(SystemParams {
            pool_index: 99,
            ignore_pause: true,
        }),
    );

    let component_editor_service = resource_container.require::<ComponentEditorService>();
    let mut component_editor_service = component_editor_service.write();

    component_editor_service.register_component("Transform2d", RegisterComponentParams::default());
    component_editor_service.register_component(
        "Translate2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    component_editor_service.register_component(
        "Rotate2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    component_editor_service.register_component(
        "Scale2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    component_editor_service.register_component("Sprite", RegisterComponentParams::default());
    component_editor_service.register_component(
        "Camera",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
}
