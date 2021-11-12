use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::state::world::WorldState;
use crate::ui_element::input::Button;
use crate::ui_element::list::ListView;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_core::entity::entity_service::EntityService;
use std::any::Any;
use std::sync::Arc;

pub fn entity_list_component() -> UIElement {
    let world_state = use_global::<WorldState>();

    let resource_container = world_state.resource_container.clone();
    let entity_service = resource_container.require::<EntityService>("entity_service");
    let entity_service = entity_service.read();

    let items: Vec<Arc<dyn Any + Send + Sync>> = entity_service
        .iter_all_entities()
        .map(|entity| Arc::new(entity) as Arc<EntitySharedRwLock>)
        .map(|entity| entity as Arc<dyn Any + Send + Sync>)
        .collect();

    ListView {
        items,
        render_item: Arc::new(|item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            let item_reader = item.read();

            let item = item.clone();
            Button {
                label: item_reader.name.clone(),
                on_click: Arc::new(move || {
                    let entity_state = use_global::<EntityState>();
                    let item = item.clone();

                    entity_state.selected_entity = Some(item.clone());
                }),
                ..Default::default()
            }
            .elem()
        }),
    }
    .elem()
}
