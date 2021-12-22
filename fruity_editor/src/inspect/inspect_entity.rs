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
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_ecs::component::component::Component;
use fruity_ecs::component::component_guard::TypedComponentReadGuard;
use fruity_ecs::component::component_guard::TypedComponentWriteGuard;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_ecs::entity::archetype::entity::Entity;
use fruity_ecs::entity::entity_service::EntityService;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, FruityAny, Clone)]
pub struct SelectEntityWrapper(pub Vec<ComponentReference>);

impl SelectEntityWrapper {
    pub fn read_component<T: Component>(&self) -> Option<TypedComponentReadGuard<T>> {
        let component = self.0.iter().find_map(|component| {
            let component_reader = component.read();
            if let Some(_) = component_reader.deref().as_any_ref().downcast_ref::<T>() {
                Some(component)
            } else {
                None
            }
        })?;

        let component = component.read();
        component.downcast::<T>()
    }

    pub fn write_component<T: Component>(&self) -> Option<TypedComponentWriteGuard<T>> {
        let component = self
            .0
            .iter()
            .find_map(|component| {
                let component_reader = component.read();
                if let Some(_) = component_reader.deref().as_any_ref().downcast_ref::<T>() {
                    Some(component)
                } else {
                    None
                }
            })
            .unwrap();

        let component = component.write();
        component.downcast::<T>()
    }
}

impl IntrospectObject for SelectEntityWrapper {
    fn get_class_name(&self) -> String {
        "SelectEntityWrapper".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for SelectEntityWrapper {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

#[topo::nested]
pub fn inspect_entity(entity_wrapper: &mut SelectEntityWrapper) -> UIElement {
    let inspector_state = use_global::<InspectorState>();
    let component_search_text = use_state(|| "".to_string());
    let display_add_component_popup = use_state(|| false);

    let world_state = use_global::<WorldState>();
    let editor_component_service = world_state
        .resource_container
        .require::<EditorComponentService>();
    let editor_component_service = editor_component_service.read();

    // TODO: Can probably be more consize with a specific Vec func
    let entity = entity_wrapper.read_component::<Entity>().unwrap();
    let entity_id = entity.entity_id;

    let other_components = entity_wrapper
        .0
        .iter()
        .filter_map(|component| {
            let component_reader = component.read();
            if let Some(_) = component_reader
                .deref()
                .as_any_ref()
                .downcast_ref::<Entity>()
            {
                None
            } else {
                Some(component.clone())
            }
        })
        .collect::<Vec<_>>();

    let entity_wrapper_2 = entity_wrapper.clone();
    let entity_wrapper_3 = entity_wrapper.clone();
    let head = Column {
        children: vec![Row {
            children: vec![
                RowItem {
                    size: UISize::Units(50.0),
                    child: Checkbox {
                        label: "".to_string(),
                        value: entity.enabled,
                        on_change: Arc::new(move |value| {
                            let mut entity = entity_wrapper_2.write_component::<Entity>().unwrap();
                            entity.enabled = value;
                        }),
                    }
                    .elem(),
                },
                RowItem {
                    size: UISize::Fill,
                    child: Input {
                        value: entity.name.to_string(),
                        placeholder: "Name ...".to_string(),
                        on_change: Arc::new(move |value: &str| {
                            let mut entity = entity_wrapper_3.write_component::<Entity>().unwrap();
                            entity.name = value.to_string();
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
        children: other_components
            .iter()
            .enumerate()
            .map(|(index, component)| {
                let component_reader = component.read();
                Collapsible {
                    title: component_reader.get_class_name(),
                    child: inspector_state.inspect_component(component.clone()),
                    secondary_actions: vec![MenuItem {
                        label: "Delete".to_string(),
                        on_click: Arc::new(move || {
                            let world_state = use_global::<WorldState>();
                            let entity_service =
                                world_state.resource_container.require::<EntityService>();
                            let entity_service = entity_service.read();
                            entity_service.remove_component(entity_id, index).ok();
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
                                .iter()
                                .map(|component| {
                                    Button {
                                        label: component.clone(),
                                        on_click: Arc::new(move || {}),
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
