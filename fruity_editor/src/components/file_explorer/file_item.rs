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
use crate::FileExplorerService;
use std::path::PathBuf;

pub fn file_item_component(path: PathBuf) -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let file_explorer_service = resource_container.require::<FileExplorerService>();
    let file_explorer_service_reader = file_explorer_service.read();

    let file_explorer_service_2 = file_explorer_service.clone();
    let path_string = path.to_str().unwrap().to_string();
    if !path.is_dir() {
        Column {
            children: vec![
                ImageButton {
                    image: file_explorer_service_reader.get_thumbnail(&path_string),
                    on_click: Arc::new(move || {
                        let file_explorer_service = file_explorer_service_2.read();
                        file_explorer_service.notify_selected(&path_string);
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
    } else {
        let path_2 = path.clone();
        Column {
            children: vec![
                ImageButton {
                    image: resource_container
                        .get::<dyn TextureResource>("Editor/Icons/folder")
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
