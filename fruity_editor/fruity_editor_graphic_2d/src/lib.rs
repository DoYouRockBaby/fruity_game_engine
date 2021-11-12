use crate::components::component::math::draw_editor_vector_2d;
use crate::gizmos_service::GizmosService;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d_untyped;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use fruity_editor::component_editor_manager::ComponentEditorManager;
use fruity_graphic_2d::math::vector2d::Vector2d;
use std::sync::Arc;

pub mod components;
pub mod gizmos_service;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let gizmos_service = GizmosService::new(resource_manager.clone());

    resource_manager
        .add::<GizmosService>("gizmos_service", Box::new(gizmos_service))
        .unwrap();

    let system_manager = resource_manager.require::<SystemManager>("system_manager");
    let mut system_manager = system_manager.write();

    system_manager.add_system_that_ignore_pause(draw_gizmos_2d_untyped, Some(98));

    let component_editor_manager =
        resource_manager.require::<ComponentEditorManager>("component_editor_manager");
    let mut component_editor_manager = component_editor_manager.write();
    component_editor_manager.register_component_field_editor::<Vector2d, _>(draw_editor_vector_2d);
}
