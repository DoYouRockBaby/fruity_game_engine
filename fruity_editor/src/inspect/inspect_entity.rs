use crate::hooks::topo;
use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::ui_element::display::Popup;
use crate::ui_element::display::Text;
use crate::ui_element::input::Button;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::Input;
use crate::ui_element::layout::Collapsible;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::RowItem;
use crate::ui_element::layout::Scroll;
use crate::ui_element::menu::MenuItem;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use crate::EditorComponentService;
use crate::InspectorState;
use crate::WorldState;
pub use comp_state::CloneState;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::entity::entity_service::EntityService;
use std::sync::Arc;

#[topo::nested]
pub fn inspect_entity(entity: &mut EntityReference) -> UIElement {
    let inspector_state = use_global::<InspectorState>();
    let component_search_text = use_state(|| "".to_string());
    let display_add_component_popup = use_state(|| false);

    let world_state = use_global::<WorldState>();
    let editor_component_service = world_state
        .resource_container
        .require::<EditorComponentService>();
    let editor_component_service = editor_component_service.read();

    let entity_reader = entity.read();
    let entity_id = entity_reader.get_entity_id();
    let entity_2 = entity.clone();
    let entity_3 = entity.clone();
    let head = Column {
        children: vec![Row {
            children: vec![
                RowItem {
                    size: UISize::Units(50.0),
                    child: Checkbox {
                        label: "".to_string(),
                        value: entity_reader.is_enabled(),
                        on_change: Arc::new(move |value| {
                            let entity_writer = entity_2.write();
                            entity_writer.set_enabled(value);
                        }),
                    }
                    .elem(),
                },
                RowItem {
                    size: UISize::Fill,
                    child: Input {
                        value: entity_reader.get_name(),
                        placeholder: "Name ...".to_string(),
                        on_change: Arc::new(move |value: &str| {
                            let entity_writer = entity_3.write();
                            entity_writer.set_name(value);
                        }),
                        ..Default::default()
                    }
                    .elem(),
                },
            ],
            ..Default::default()
        }
        .elem()],
        align: UIAlign::default(),
    }
    .elem();

    let components = Column {
        children: entity
            .iter_all_component()
            .enumerate()
            .map(|(index, component)| {
                let class_name = {
                    let component_reader = component.read();
                    component_reader.get_class_name()
                };

                Collapsible {
                    title: class_name,
                    child: inspector_state.inspect_component(component),
                    secondary_actions: vec![MenuItem {
                        label: "Delete".to_string(),
                        on_click: Arc::new(move || {
                            // Get what we need
                            let world_state = use_global::<WorldState>();
                            let inspector_state = use_global::<InspectorState>();
                            let entity_service =
                                world_state.resource_container.require::<EntityService>();
                            let entity_service = entity_service.read();

                            // Remove the component
                            entity_service.remove_component(entity_id, index).ok();

                            // Update the selected entity reference
                            inspector_state
                                .select(Box::new(entity_service.get_entity(entity_id).unwrap()));
                        }),
                    }],
                    ..Default::default()
                }
                .elem()
            })
            .collect::<Vec<_>>(),
        align: UIAlign::Start,
    }
    .elem();

    let add_component = Column {
        children: vec![Button {
            label: "+".to_string(),
            on_click: Arc::new(move || {
                display_add_component_popup.set(!display_add_component_popup.get())
            }),
            ..Default::default()
        }
        .elem()],
        align: UIAlign::Center,
    }
    .elem();

    let add_component_popup = if display_add_component_popup.get() {
        Popup {
            content: Column {
                children: vec![
                    Row {
                        children: vec![
                            RowItem {
                                size: UISize::Units(40.0),
                                child: Text {
                                    text: "🔍".to_string(),
                                    ..Default::default()
                                }
                                .elem(),
                            },
                            RowItem {
                                size: UISize::Fill,
                                child: Input {
                                    value: component_search_text.get(),
                                    placeholder: "Search ...".to_string(),
                                    on_edit: Arc::new(move |value| {
                                        component_search_text.set(value.to_string());
                                    }),
                                    ..Default::default()
                                }
                                .elem(),
                            },
                        ],
                        ..Default::default()
                    }
                    .elem(),
                    Scroll {
                        child: Column {
                            children: editor_component_service
                                .search(&component_search_text.get())
                                .map(|component| {
                                    Button {
                                        label: component.clone(),
                                        on_click: Arc::new(move || {
                                            // Get what we need
                                            let world_state = use_global::<WorldState>();
                                            let inspector_state = use_global::<InspectorState>();
                                            let entity_service = world_state
                                                .resource_container
                                                .require::<EntityService>();
                                            let entity_service = entity_service.read();
                                            let editor_component_service = world_state
                                                .resource_container
                                                .require::<EditorComponentService>();
                                            let editor_component_service =
                                                editor_component_service.read();

                                            // Add the component
                                            if let Some(components) =
                                                editor_component_service.instantiate(&component)
                                            {
                                                entity_service
                                                    .add_component(entity_id, components)
                                                    .ok();
                                            }

                                            // Update the selected entity reference
                                            inspector_state.select(Box::new(
                                                entity_service.get_entity(entity_id).unwrap(),
                                            ));
                                        }),
                                        ..Default::default()
                                    }
                                    .elem()
                                })
                                .collect::<Vec<_>>(),
                            ..Default::default()
                        }
                        .elem(),
                        ..Default::default()
                    }
                    .elem(),
                ],
                align: UIAlign::Start,
            }
            .elem(),
        }
        .elem()
    } else {
        Empty {}.elem()
    };

    Scroll {
        child: Column {
            children: vec![head, components, add_component, add_component_popup],
            align: UIAlign::Start,
        }
        .elem(),
        ..Default::default()
    }
    .elem()
}
