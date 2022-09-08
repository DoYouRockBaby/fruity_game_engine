use crate::components::file_explorer::TextureResource;
use crate::state::file_explorer::FileExplorerState;
use crate::ui::context::UIContext;
use crate::ui::elements::display::Text;
use crate::ui::elements::input::ImageButton;
use crate::ui::elements::layout::Column;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_service;
use crate::ui::hooks::use_write_service;
use crate::Arc;
use crate::FileExplorerService;
use fruity_any::FruityAny;
use fruity_core::utils::string::get_file_type_from_path;
use std::path::PathBuf;

pub fn file_item_component(ctx: &mut UIContext, path: PathBuf) -> UIElement {
    let resource_container = ctx.resource_container();
    let file_explorer_service = use_service::<FileExplorerService>(ctx);
    let file_explorer_service_reader = file_explorer_service.read();

    let file_explorer_service_2 = file_explorer_service.clone();
    let path_string = path.to_str().unwrap().to_string();
    if !path.is_dir() {
        if let Some(extension) = get_file_type_from_path(&path_string) {
            if !resource_container.contains(&path_string) {
                resource_container
                    .load_resource_file(&path_string, &extension)
                    .ok();
            }

            let resource = resource_container.get_untyped(&path_string);

            Column {
                children: vec![
                    ImageButton {
                        image: file_explorer_service_reader.get_thumbnail(ctx, &path_string),
                        enabled: true,
                        drag_item: resource.map(|resource| Arc::new(resource.clone()).as_any_arc()),
                        width: 64.0,
                        height: 64.0,
                        on_click: Arc::new(move |ctx| {
                            let file_explorer_service = file_explorer_service_2.read();
                            file_explorer_service.notify_selected(ctx, &path_string);
                        }),
                        accept_drag: None,
                        on_drag: None,
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
            Empty {}.elem()
        }
    } else {
        let path_2 = path.clone();
        Column {
            children: vec![
                ImageButton {
                    image: resource_container
                        .get::<dyn TextureResource>("Editor/Icons/folder")
                        .unwrap(),
                    enabled: true,
                    width: 64.0,
                    height: 64.0,
                    on_click: Arc::new(move |ctx| {
                        let mut file_explorer_state = use_write_service::<FileExplorerState>(ctx);
                        file_explorer_state.open_dir(&path_2.to_path_buf().to_string_lossy());
                    }),
                    drag_item: None,
                    accept_drag: None,
                    on_drag: None,
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
