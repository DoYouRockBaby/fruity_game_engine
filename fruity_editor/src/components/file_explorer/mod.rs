use crate::components::file_explorer::file_item::file_item_component;
use crate::hooks::use_global;
use crate::state::file_explorer::FileExplorerState;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::RowItem;
use crate::ui_element::layout::Scroll;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use fruity_graphic::resources::texture_resource::TextureResource;

pub mod file_item;

pub fn file_explorer_component() -> UIElement {
    let file_explorer_state = use_global::<FileExplorerState>();
    let files = file_explorer_state.get_files();

    Scroll {
        child: Row {
            children: files
                .into_iter()
                .map(|file| RowItem {
                    size: UISize::Units(64.0),
                    child: file_item_component(file),
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        }
        .elem(),
        ..Default::default()
    }
    .elem()
}
