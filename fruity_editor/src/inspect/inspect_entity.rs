use crate::components::fields::edit_introspect_fields;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::Input;
use crate::ui_element::layout::Collapsible;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::RowItem;
use crate::ui_element::layout::Scroll;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
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
use std::ops::Deref;
use std::sync::Arc;

// TODO: Try to remove this struct
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

pub fn inspect_entity(entity_wrapper: &mut SelectEntityWrapper) -> UIElement {
    // TODO: Can probably be more consize with a specific Vec func
    let entity = entity_wrapper.read_component::<Entity>().unwrap();

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
            .map(|component| {
                let component_reader = component.read();
                Collapsible {
                    title: component_reader.get_class_name(),
                    on_click: None,
                    child: edit_introspect_fields(Box::new(component.clone())),
                }
                .elem()
            })
            .collect::<Vec<_>>(),
        align: UIAlign::Start,
    }
    .elem();

    Scroll {
        child: Column {
            children: vec![head, components],
            align: UIAlign::Start,
        }
        .elem(),
        ..Default::default()
    }
    .elem()
}
