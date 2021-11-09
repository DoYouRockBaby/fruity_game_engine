use crate::components::file_explorer::ResourceIdentifier;
use crate::components::file_explorer::TextureResource;
use crate::hooks::use_global;
use crate::state::file_explorer::FileExplorerState;
use crate::state::world::WorldState;
use crate::ui_element::display::Text;
use crate::ui_element::input::ImageButton;
use crate::ui_element::layout::Column;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use crate::Arc;
use crate::FileExplorerManager;
use fruity_core::resource::resources_manager::ResourcesManager;
use std::path::PathBuf;

pub fn file_item_component(path: PathBuf) -> UIElement {
    let world_state = use_global::<WorldState>();
    let service_manager = world_state.service_manager.read().unwrap();
    let file_explorer_manager = service_manager.read::<FileExplorerManager>();
    let resource_manager = service_manager.read::<ResourcesManager>();

    if !path.is_dir() {
        Column {
            children: vec![
                ImageButton {
                    image: file_explorer_manager.get_thumbnail(path.to_str().unwrap()),
                    on_click: Arc::new(move || {}),
                    width: 64.0,
                    height: 64.0,
                }
                .elem(),
                Text {
                    text: path.file_name().unwrap().to_string_lossy().to_string(),
                    ..Default::default()
                }
                .elem(),
            ],
            ..Default::default()
        }
        .elem()
    } else {
        let path_2 = path.clone();
        Column {
            children: vec![
                ImageButton {
                    image: resource_manager
                        .get_resource::<TextureResource>(ResourceIdentifier(
                            "Editor/Icons/folder".to_string(),
                        ))
                        .unwrap(),
                    on_click: Arc::new(move || {
                        let file_explorer_state = use_global::<FileExplorerState>();

                        file_explorer_state.current_dir =
                            path_2.to_path_buf().to_string_lossy().to_string();
                    }),
                    width: 64.0,
                    height: 64.0,
                }
                .elem(),
                Text {
                    text: path.file_name().unwrap().to_string_lossy().to_string(),
                    ..Default::default()
                }
                .elem(),
            ],
            ..Default::default()
        }
        .elem()
    }
}
