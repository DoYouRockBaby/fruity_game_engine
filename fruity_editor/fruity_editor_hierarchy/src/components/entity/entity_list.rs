use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::entity::archetype::entity::Entity;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_editor::hooks::use_global;
use fruity_editor::inspect::inspect_entity::SelectEntityWrapper;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::state::world::WorldState;
use fruity_editor::ui_element::input::Button;
use fruity_editor::ui_element::layout::Collapsible;
use fruity_editor::ui_element::layout::Column;
use fruity_editor::ui_element::layout::Scroll;
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
        .map(|components| Arc::new(SelectEntityWrapper(components)))
        .collect::<Vec<_>>();

    let root_entities = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity.read_component::<Parent>() {
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
    entity: Arc<SelectEntityWrapper>,
    all_entities: &Vec<Arc<SelectEntityWrapper>>,
    entity_service: ResourceReference<EntityService>,
) -> UIElement {
    let entity = entity.read_component::<Entity>().unwrap();
    let entity_id = entity.entity_id;

    let children = all_entities
        .iter()
        .filter(|entity| {
            if let Some(parent) = entity.read_component::<Parent>() {
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
            title: entity.name.clone(),
            on_click: Some(Arc::new(move || {
                let inspector_state = use_global::<InspectorState>();
                let entity_service_reader = entity_service.read();
                let full_entity = entity_service_reader.get_full_entity(entity_id);

                if let Some(full_entity) = full_entity {
                    inspector_state.select(Box::new(SelectEntityWrapper(full_entity)));
                }
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
            ..Default::default()
        }
        .elem()
    } else {
        Button {
            label: entity.name.clone(),
            on_click: Arc::new(move || {
                let inspector_state = use_global::<InspectorState>();
                let entity_service_reader = entity_service.read();
                let full_entity = entity_service_reader.get_full_entity(entity_id);

                if let Some(full_entity) = full_entity {
                    inspector_state.select(Box::new(SelectEntityWrapper(full_entity)));
                }
            }),
            ..Default::default()
        }
        .elem()
    }
}
