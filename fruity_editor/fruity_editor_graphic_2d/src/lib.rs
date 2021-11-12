use crate::components::component::math::draw_editor_vector_2d;
use crate::gizmos_service::GizmosService;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d_untyped;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_core::system::system_service::SystemService;
use fruity_editor::component_editor_service::ComponentEditorService;
use fruity_graphic_2d::math::vector2d::Vector2d;
use std::sync::Arc;

pub mod components;
pub mod gizmos_service;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let gizmos_service = GizmosService::new(resource_container.clone());

    resource_container
        .add::<GizmosService>("gizmos_service", Box::new(gizmos_service))
        .unwrap();

    let system_service = resource_container.require::<SystemService>("system_service");
    let mut system_service = system_service.write();

    system_service.add_system_that_ignore_pause(draw_gizmos_2d_untyped, Some(98));

    let component_editor_service =
        resource_container.require::<ComponentEditorService>("component_editor_service");
    let mut component_editor_service = component_editor_service.write();
    component_editor_service.register_component_field_editor::<Vector2d, _>(draw_editor_vector_2d);
}
