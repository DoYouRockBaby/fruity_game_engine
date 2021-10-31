use crate::components::component::edit_component_component;
use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;

pub fn entity_edit_component() -> UIElement {
    let entity_state = use_global::<EntityState>();

    if let Some(entity) = &entity_state.selected_entity {
        let entity_reader = entity.read();

        let head = UIElement::Column {
            children: vec![UIElement::Row {
                children: vec![
                    UIElement::Checkbox {
                        label: "".to_string(),
                        value: entity_reader.enabled,
                        on_change: Box::new(move |value| {
                            let entity = entity.clone();
                            let mut entity = entity.write();
                            entity.enabled = value;
                        }),
                    },
                    UIElement::Input {
                        label: "".to_string(),
                        value: entity_reader.name.to_string(),
                        placeholder: "Name ...".to_string(),
                        on_change: Box::new(move |value| {
                            let entity = entity.clone();
                            let mut entity = entity.write();
                            entity.name = value.to_string();
                        }),
                    },
                ],
                align: UIAlign::Center,
            }],
            align: UIAlign::default(),
        };

        let components = UIElement::Column {
            children: entity
                .iter_all_components()
                .map(|component| {
                    let component_reader = component.read();
                    UIElement::Column {
                        children: vec![
                            UIElement::Text(component_reader.get_component_type()),
                            edit_component_component(component.clone()),
                        ],
                        align: UIAlign::Start,
                    }
                })
                .collect::<Vec<_>>(),
            align: UIAlign::Start,
        };

        UIElement::Column {
            children: vec![head, components],
            align: UIAlign::Start,
        }
    } else {
        UIElement::Empty
    }
}
