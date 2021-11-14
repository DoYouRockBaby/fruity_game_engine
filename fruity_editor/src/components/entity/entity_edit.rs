use crate::components::fields::edit_component_fields;
use crate::hooks::use_global;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::Input;
use crate::ui_element::layout::Collapsible;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::layout::Row;
use crate::ui_element::layout::RowItem;
use crate::ui_element::layout::Scroll;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use crate::WorldState;
use std::sync::Arc;

pub fn entity_edit_component() -> UIElement {
    let world_state = use_global::<WorldState>();

    if let Some(entity) = &world_state.selected_entity {
        let entity_reader = entity.read();

        let head = Column {
            children: vec![Row {
                children: vec![
                    RowItem {
                        size: UISize::Units(50.0),
                        child: Checkbox {
                            label: "".to_string(),
                            value: entity_reader.enabled,
                            on_change: Arc::new(move |value| {
                                let entity = entity.clone();
                                let mut entity = entity.write();
                                entity.enabled = value;
                            }),
                        }
                        .elem(),
                    },
                    RowItem {
                        size: UISize::Fill,
                        child: Input {
                            value: entity_reader.name.to_string(),
                            placeholder: "Name ...".to_string(),
                            on_change: Arc::new(move |value: &str| {
                                let entity = entity.clone();
                                let mut entity = entity.write();
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
            children: entity
                .iter_all_components()
                .map(|component| {
                    let component_reader = component.read();
                    Collapsible {
                        title: component_reader.get_component_type(),
                        child: edit_component_fields(component.clone()),
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
    } else {
        Empty {}.elem()
    }
}
