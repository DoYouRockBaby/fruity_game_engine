use crate::components::file_explorer::file_item::file_item_component;
use crate::state::file_explorer::FileExplorerState;
use crate::ui::context::UIContext;
use crate::ui::elements::layout::Row;
use crate::ui::elements::layout::RowItem;
use crate::ui::elements::layout::Scroll;
use crate::ui::elements::UIElement;
use crate::ui::elements::UISize;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use fruity_graphic::resources::texture_resource::TextureResource;

pub mod file_item;

pub fn file_explorer_component(ctx: &mut UIContext) -> UIElement {
    let file_explorer_state = use_read_service::<FileExplorerState>(ctx);
    let files = file_explorer_state.get_files();

    Scroll {
        child: Row {
            children: files
                .into_iter()
                .map(|file| RowItem {
                    size: UISize::Units(64.0),
                    child: file_item_component(ctx, file),
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        }
        .elem(),
        ..Default::default()
    }
    .elem()
}
