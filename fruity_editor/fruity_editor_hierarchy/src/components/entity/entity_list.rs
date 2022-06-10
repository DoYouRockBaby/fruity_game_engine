use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::state::world::WorldState;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::layout::Collapsible;
use fruity_editor::ui_element::layout::Column;
use fruity_editor::ui_element::layout::Scroll;
use fruity_editor::ui_element::menu::MenuItem;
use fruity_editor::ui_element::UIElement;
use fruity_editor::ui_element::UIWidget;
use fruity_hierarchy::components::parent::Parent;
use std::sync::Arc;

pub fn entity_list_component() -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let entity_service = resource_container.require::<EntityService>();
    let entity_service_reader = entity_service.read();

    let all_entities = entity_service_reader
        .iter_all_entities()
        .collect::<Vec<_>>();

    let root_entities = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity.read().read_single_component::<Parent>() {
                if let Some(_) = *parent.parent_id {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    Scroll {
        child: Column {
            children: root_entities
                .iter()
                .map(|child| {
                    draw_entity_line((*child).clone(), &all_entities, entity_service.clone())
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        }
        .elem(),
        ..Default::default()
    }
    .elem()
}

pub fn draw_entity_line(
    entity: EntityReference,
    all_entities: &Vec<EntityReference>,
    entity_service: ResourceReference<EntityService>,
) -> UIElement {
    let entity_2 = entity.clone();
    let entity_3 = entity.clone();
    let entity_reader = entity.read();
    let entity_id = entity_reader.get_entity_id();

    let children = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity.read().read_single_component::<Parent>() {
                if let Some(parent_id) = *parent.parent_id {
                    parent_id == entity_id
                } else {
                    false
                }
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    if children.len() > 0 {
        let entity_service_2 = entity_service.clone();
        Collapsible {
            key: entity_reader.get_name(),
            title: entity_reader.get_name(),
            on_click: Some(Arc::new(move || {
                let inspector_state = use_global::<InspectorState>();
                inspector_state.select(Box::new(entity_2.clone()));
            })),
            child: Column {
                children: children
                    .iter()
                    .map(|child| {
                        draw_entity_line((*child).clone(), all_entities, entity_service_2.clone())
                    })
                    .collect::<Vec<_>>(),
                ..Default::default()
            }
            .elem(),
            secondary_actions: vec![MenuItem {
                label: "Delete".to_string(),
                on_click: Arc::new(move || {
                    let world_state = use_global::<WorldState>();
                    let entity_service = world_state.resource_container.require::<EntityService>();
                    let entity_service = entity_service.read();
                    entity_service.remove(entity_id).ok();
                }),
                options: Default::default(),
            }],
            ..Default::default()
        }
        .elem()
    } else {
        Button {
            label: entity_reader.get_name(),
            on_click: Arc::new(move || {
                let inspector_state = use_global::<InspectorState>();
                inspector_state.select(Box::new(entity_3.clone()));
            }),
            secondary_actions: vec![MenuItem {
                label: "Delete".to_string(),
                on_click: Arc::new(move || {
                    let world_state = use_global::<WorldState>();
                    let entity_service = world_state.resource_container.require::<EntityService>();
                    let entity_service = entity_service.read();
                    entity_service.remove(entity_id).ok();
                }),
                options: Default::default(),
            }],
            ..Default::default()
        }
        .elem()
    }
}
