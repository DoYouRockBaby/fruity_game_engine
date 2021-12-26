use crate::Parent;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::ops::Deref;

pub fn delete_cascade(entity_service: Ref<EntityService>) {
    // TODO: Disable observer on system end
    let entity_service_reader = entity_service.read();
    entity_service_reader
        .on_deleted
        .add_observer(move |parent_id| {
            let parent_id = *parent_id;
            let entity_service = entity_service.clone();
            let entity_service_reader = entity_service.read();

            entity_service_reader.for_each(
                entity_type!["Parent"],
                Inject2::new(move |entity_id: EntityId, parent: Read<Parent>| {
                    let is_child_of_deleted = {
                        if let Some(entity_parent_id) = parent.parent_id.deref() {
                            *entity_parent_id == parent_id
                        } else {
                            false
                        }
                    };

                    std::mem::drop(parent);

                    if is_child_of_deleted {
                        let entity_service = entity_service.read();
                        entity_service.remove(entity_id).ok();
                    }
                }),
            )
        });
}
