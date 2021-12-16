use crate::editor_menu_service::EditorMenuService;
use crate::hooks::use_global;
use crate::ui_element::menu::MenuItem;
use crate::ui_element::menu::MenuSection;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::WorldState;

pub fn menu_sections_component() -> Vec<UIElement> {
    let world_state = use_global::<WorldState>();
    let editor_menu_service = world_state
        .resource_container
        .require::<EditorMenuService>();
    let editor_menu_service = editor_menu_service.read();

    editor_menu_service
        .iter_sections()
        .map(|section| {
            MenuSection {
                label: section.0.clone(),
                items: section
                    .1
                    .iter()
                    .map(|menu_item| MenuItem {
                        label: menu_item.0.clone(),
                        on_click: menu_item.1.clone(),
                    })
                    .collect::<Vec<_>>(),
            }
            .elem()
        })
        .collect::<Vec<_>>()
}
