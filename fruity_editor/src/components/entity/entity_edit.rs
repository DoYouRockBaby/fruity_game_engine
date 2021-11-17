use crate::components::fields::edit_component_fields;
use crate::hooks::use_global;
use crate::state::select_entity::SelectEntityState;
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
use std::sync::Arc;

pub fn entity_edit_component() -> UIElement {
    let select_entity_state = use_global::<SelectEntityState>();

    if let Some(entity) = select_entity_state.get_selected_entity() {
        let entity_reader = entity.read();

        let entity_2 = entity.clone();
        let entity_3 = entity.clone();
        let head = Column {
            children: vec![Row {
                children: vec![
                    RowItem {
                        size: UISize::Units(50.0),
                        child: Checkbox {
                            label: "".to_string(),
                            value: entity_reader.enabled,
                            on_change: Arc::new(move |value| {
                                let mut entity = entity_2.write();
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
                                let mut entity = entity_3.write();
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
                        title: component_reader.get_class_name(),
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
