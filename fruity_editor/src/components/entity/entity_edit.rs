use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::ui_element::UIElement;

pub fn entity_edit_component() -> UIElement {
    let entity_state = use_global::<EntityState>();

    if let Some(entity) = &entity_state.selected_entity {
        let entity_reader = entity.read();

        UIElement::Column(vec![UIElement::Row(vec![
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
        ])])
    } else {
        UIElement::Empty
    }
}
