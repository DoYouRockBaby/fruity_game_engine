use crate::gizmos_service::GizmosService;
use crate::systems::display_grid::display_grid;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d;
use fruity_core::inject::Inject3;
use fruity_core::inject::Inject4;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;

pub mod gizmos_service;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_graphic_2d";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let gizmos_service = GizmosService::new(resource_container.clone());

    resource_container.add::<GizmosService>("gizmos_service", Box::new(gizmos_service));

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "draw_gizmos_2d",
        MODULE_NAME,
        Inject4::new(draw_gizmos_2d),
        SystemParams {
            pool_index: 98,
            ignore_pause: true,
        },
    );

    system_service.add_system(
        "display_grid",
        MODULE_NAME,
        Inject3::new(display_grid),
        SystemParams {
            pool_index: 98,
            ignore_pause: true,
        },
    );

    let editor_component_service = resource_container.require::<EditorComponentService>();
    let mut editor_component_service = editor_component_service.write();

    editor_component_service.register_component("Transform2d", RegisterComponentParams::default());
    editor_component_service.register_component(
        "Translate2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    editor_component_service.register_component(
        "Rotate2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    editor_component_service.register_component(
        "Scale2d",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
    editor_component_service.register_component("Sprite", RegisterComponentParams::default());
    editor_component_service.register_component(
        "Camera",
        RegisterComponentParams {
            dependencies: vec!["Transform2d".to_string()],
            ..Default::default()
        },
    );
}
