use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::state::world::WorldState;
use crate::ui_element::display::Text;
use crate::ui_element::list::ListView;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_core::entity::entity_manager::EntityManager;
use std::any::Any;
use std::sync::Arc;

pub fn entity_list_component() -> UIElement {
    let world_state = use_global::<WorldState>();

    let service_manager = world_state.service_manager.clone();
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    let items: Vec<Arc<dyn Any + Send + Sync>> = entity_manager
        .iter_all_entities()
        .map(|entity| Arc::new(entity) as Arc<EntitySharedRwLock>)
        .map(|entity| entity as Arc<dyn Any + Send + Sync>)
        .collect();

    ListView {
        items,
        get_key: Box::new(|item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            let item = item.read();
            item.entity_id as usize
        }),
        render_item: Box::new(|item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            let item = item.read();
            Text {
                text: item.name.clone(),
                ..Text::default()
            }
            .elem()
        }),
        on_clicked: Arc::new(move |item: &dyn Any| {
            let entity_state = use_global::<EntityState>();

            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            entity_state.selected_entity = Some(item.clone());
        }),
    }
    .elem()
}
