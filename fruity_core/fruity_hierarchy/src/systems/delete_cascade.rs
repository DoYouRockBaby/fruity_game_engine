use crate::Parent;
use fruity_core::inject::Ref;
use fruity_core::signal::ObserverIdentifier;
use fruity_ecs::entity::archetype::entity::Entity;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

pub fn delete_cascade(entity_service: Ref<EntityService>) {
    let entity_observer_identifiers =
        Arc::new(Mutex::new(HashMap::<EntityId, ObserverIdentifier>::new()));

    let entity_service_reader = entity_service.read();
    entity_service_reader.for_each(
        entity_type!["Entity", "Parent"],
        Inject2::new(move |entity: Read<Entity>, parent: Read<Parent>| {
            let entity_id = entity.entity_id;
            let entity_observer_identifiers_2 = entity_observer_identifiers.clone();

            // Get the parent entity reference
            let parent_components = if let Some(parent_id) = &parent.parent_id.deref() {
                let entity_service_reader = entity_service.read();
                entity_service_reader.get_entity(*parent_id, entity_type!["Entity"])
            } else {
                None
            };

            // If parent is deleted, delete itself
            if let Some(parent_components) = parent_components {
                let parent_entity = parent_components.get(0).unwrap();
                let parent_entity = parent_entity.read();
                let parent_entity = parent_entity.as_any_ref().downcast_ref::<Entity>().unwrap();

                let entity_service = entity_service.clone();
                let observer_id = parent_entity.on_deleted.add_observer(move |_| {
                    let entity_service = entity_service.read();
                    entity_service.remove(entity_id).ok();
                });

                let mut entity_observer_identifiers = entity_observer_identifiers.lock().unwrap();
                entity_observer_identifiers.insert(entity.entity_id, observer_id);
            }

            // When parent is updated, we update the parent delete observer
            let entity_service = entity_service.clone();
            parent.parent_id.on_updated.add_observer(move |parent_id| {
                let entity_observer_identifiers_2 = entity_observer_identifiers_2.clone();

                // Get the parent entity reference
                let parent_components = if let Some(parent_id) = &parent_id {
                    let entity_service_reader = entity_service.read();
                    entity_service_reader.get_entity(*parent_id, entity_type!["Entity"])
                } else {
                    None
                };

                let entity_service = entity_service.clone();

                if let Some(parent_components) = parent_components {
                    let parent_entity = parent_components.get(0).unwrap();
                    let parent_entity = parent_entity.read();
                    let parent_entity =
                        parent_entity.as_any_ref().downcast_ref::<Entity>().unwrap();

                    // Disabled the old observer
                    {
                        let mut entity_observer_identifiers =
                            entity_observer_identifiers_2.lock().unwrap();
                        if let Some(observer_id) = entity_observer_identifiers.remove(&entity_id) {
                            parent_entity.on_deleted.remove_observer(observer_id);
                        }
                    }

                    // Add the new observer
                    let observer_id = parent_entity.on_deleted.add_observer(move |_| {
                        let entity_service = entity_service.read();
                        entity_service.remove(entity_id).ok();
                    });

                    let mut entity_observer_identifiers =
                        entity_observer_identifiers_2.lock().unwrap();
                    entity_observer_identifiers.insert(entity_id, observer_id);
                }
            });
        }),
    )
}
