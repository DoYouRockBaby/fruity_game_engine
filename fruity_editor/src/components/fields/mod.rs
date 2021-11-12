use crate::hooks::use_global;
use crate::state::world::WorldState;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::ComponentEditorManager;
use fruity_core::component::component_rwlock::ComponentRwLock;

pub mod primitive;

pub fn edit_component_fields(component: ComponentRwLock) -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_manager = world_state.resource_manager.clone();
    let component_editor_manager =
        resource_manager.require::<ComponentEditorManager>("component_editor_manager");
    let component_editor_manager = component_editor_manager.read();

    let reader = component.read();
    let fields_edit = reader
        .get_field_infos()
        .iter()
        .map(|field_info| {
            if let Some(field_editor) =
                component_editor_manager.get_component_field_editor(field_info.ty)
            {
                field_editor(component.clone(), field_info)
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
