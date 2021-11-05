use crate::components::component::edit_component_fields;
use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::ui_element::display::Text;
use crate::ui_element::input::Checkbox;
use crate::ui_element::input::Input;
use crate::ui_element::layout::Column;
use crate::ui_element::layout::Empty;
use crate::ui_element::layout::Row;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub fn entity_edit_component() -> UIElement {
    let entity_state = use_global::<EntityState>();

    if let Some(entity) = &entity_state.selected_entity {
        let entity_reader = entity.read();

        let head = Column {
            children: vec![Row {
                children: vec![
                    Checkbox {
                        label: "".to_string(),
                        value: entity_reader.enabled,
                        on_change: Arc::new(move |value| {
                            let entity = entity.clone();
                            let mut entity = entity.write();
                            entity.enabled = value;
                        }),
                    }
                    .elem(),
                    Input {
                        value: entity_reader.name.to_string(),
                        placeholder: "Name ...".to_string(),
                        on_change: Arc::new(move |value: &str| {
                            let entity = entity.clone();
                            let mut entity = entity.write();
                            entity.name = value.to_string();
                        }),
                    }
                    .elem(),
                ],
                align: UIAlign::Center,
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
                    Column {
                        children: vec![
                            Text {
                                text: component_reader.get_component_type(),
                                ..Text::default()
                            }
                            .elem(),
                            edit_component_fields(component.clone()),
                        ],
                        align: UIAlign::Start,
                    }
                    .elem()
                })
                .collect::<Vec<_>>(),
            align: UIAlign::Start,
        }
        .elem();

        Column {
            children: vec![head, components],
            align: UIAlign::Start,
        }
        .elem()
    } else {
        Empty {}.elem()
    }
}
