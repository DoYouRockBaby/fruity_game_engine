use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::IntrospectEditorService;
use fruity_core::serialize::serialized::SerializableObject;
use std::ops::Deref;

pub mod primitive;

pub fn edit_introspect_fields(introspect_object: Box<dyn SerializableObject>) -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let introspect_editor_service = resource_container.require::<IntrospectEditorService>();
    let introspect_editor_service = introspect_editor_service.read();

    let fields_edit = introspect_object
        .deref()
        .get_field_infos()
        .iter()
        .map(|field_info| {
            if let Some(field_editor) = introspect_editor_service.get_field_editor(field_info.ty) {
                let introspect_object = introspect_object.duplicate();
                field_editor(introspect_object, field_info)
            } else {
                Empty {}.elem()
            }
        })
        .collect::<Vec<_>>();

    Column {
        children: fields_edit,
        align: UIAlign::Start,
    }
    .elem()
}
