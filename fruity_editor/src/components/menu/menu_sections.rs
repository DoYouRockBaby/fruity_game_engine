use crate::editor_menu_service::EditorMenuService;
use crate::editor_menu_service::MenuItem;
use crate::ui::context::UIContext;
use crate::ui::elements::menu::MenuSection;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;

pub fn menu_sections_component(ctx: &mut UIContext) -> Vec<UIElement> {
    let editor_menu_service = use_read_service::<EditorMenuService>(ctx);

    editor_menu_service
        .iter_sections()
        .map(|section| {
            MenuSection {
                label: section.0.clone(),
                items: section
                    .1
                    .iter()
                    .map(|menu_item| MenuItem {
                        label: menu_item.label.clone(),
                        action: menu_item.action.clone(),
                        options: menu_item.options.clone(),
                    })
                    .collect::<Vec<_>>(),
            }
            .elem()
        })
        .collect::<Vec<_>>()
}
