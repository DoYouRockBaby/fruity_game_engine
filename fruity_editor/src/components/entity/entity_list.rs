use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::state::world::WorldState;
use crate::ui_element::UIElement;
use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_core::entity::entity_manager::EntityManager;
use std::any::Any;

pub fn entity_list_component() -> UIElement {
    let world_state = use_global::<WorldState>();
    let entity_state = use_global::<EntityState>();

    let service_manager = world_state.service_manager.clone();
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    let items: Vec<Box<dyn Any + Send + Sync>> = entity_manager
        .iter_all_entities()
        .map(|entity| Box::new(entity) as Box<EntitySharedRwLock>)
        .map(|entity| entity as Box<dyn Any + Send + Sync>)
        .collect();

    UIElement::ListView {
        items,
        get_key: Box::new(|item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            let item = item.read();
            item.entity_id as usize
        }),
        render_item: Box::new(|item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            let item = item.read();
            UIElement::Text(item.name.clone())
        }),
        on_clicked: Box::new(move |item: &dyn Any| {
            let item = item.downcast_ref::<EntitySharedRwLock>().unwrap();
            entity_state.selected_entity = Some(item.clone());
        }),
    }
}
