use crate::Parent;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::ops::Deref;

pub fn update_nested_level(entity_service: Ref<EntityService>) {
    let entity_service_reader = entity_service.read();
    entity_service_reader.for_each(
        entity_type!["Parent"],
        Inject2::new(move |entity: EntityReference, parent: &mut Parent| {
            // Get the parent entity reference
            let parent_entity = if let Some(parent_id) = &parent.parent_id.deref() {
                let entity_service_reader = entity_service.read();
                entity_service_reader.get_entity(*parent_id)
            } else {
                None
            };

            // Set the nested level as the parent one plus one
            if let Some(parent_entity) = parent_entity {
                let parent_entity = parent_entity.read();
                if let Some(parent_parent) =
                    parent_entity.read_single_typed_component::<Parent>("Parent")
                {
                    parent.nested_level = parent_parent.nested_level + 1;
                } else {
                    parent.nested_level = 1;
                }
            }

            // When parent is updated, we update the nested level
            let entity_2 = entity.clone();
            let entity_service = entity_service.clone();
            parent.parent_id.on_updated.add_observer(move |parent_id| {
                let entity = entity_2.write();
                let mut parent = entity
                    .write_single_typed_component::<Parent>("Parent")
                    .unwrap();

                // Get the parent entity reference
                let parent_entity = if let Some(parent_id) = &parent_id {
                    let entity_service_reader = entity_service.read();
                    entity_service_reader.get_entity(*parent_id)
                } else {
                    None
                };

                // Set the nested level as the parent one plus one
                if let Some(parent_entity) = parent_entity {
                    let parent_entity = parent_entity.read();
                    if let Some(parent_parent) =
                        parent_entity.read_single_typed_component::<Parent>("Parent")
                    {
                        parent.nested_level = parent_parent.nested_level + 1;
                    } else {
                        parent.nested_level = 1;
                    }
                }
            });
        }),
    )
}
