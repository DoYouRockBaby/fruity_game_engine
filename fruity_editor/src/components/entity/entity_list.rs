use crate::hooks::use_global;
use crate::inspect::inspect_entity::SelectEntityWrapper;
use crate::state::inspector::InspectorState;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::list::ListView;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_ecs::entity::archetype::entity::Entity;
use fruity_ecs::entity::entity_service::EntityService;
use std::any::Any;
use std::sync::Arc;

pub fn entity_list_component() -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let entity_service = resource_container.require::<EntityService>();
    let entity_service_reader = entity_service.read();

    let items: Vec<Arc<dyn Any + Send + Sync>> = entity_service_reader
        .iter_all_entities()
        .map(|components| Arc::new(SelectEntityWrapper(components)))
        .map(|entity| entity as Arc<dyn Any + Send + Sync>)
        .collect();

    ListView {
        items,
        render_item: Arc::new(move |item: &dyn Any| {
            let item = item.downcast_ref::<SelectEntityWrapper>().unwrap();
            let entity = item.read_component::<Entity>().unwrap();
            let entity_id = entity.entity_id;

            let entity_service = entity_service.clone();
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
        }),
    }
    .elem()
}
