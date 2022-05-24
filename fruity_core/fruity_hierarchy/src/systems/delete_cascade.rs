use crate::Parent;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithEntity;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::entity::entity_service::EntityService;
use std::ops::Deref;

pub fn delete_cascade(
    entity_service: Ref<EntityService>,
    query: Query<(WithEntity, With<Parent>)>,
) {
    // TODO: Disable observer on system end
    let entity_service_reader = entity_service.read();
    entity_service_reader
        .on_deleted
        .add_observer(move |parent_id| {
            let parent_id = *parent_id;
            let entity_service = entity_service.clone();

            query.for_each(move |(entity, parent)| {
                let is_child_of_deleted = {
                    if let Some(entity_parent_id) = parent.parent_id.deref() {
                        *entity_parent_id == parent_id
                    } else {
                        false
                    }
                };

                std::mem::drop(parent);

                if is_child_of_deleted {
                    let entity_id = {
                        let entity = entity.read();
                        entity.get_entity_id()
                    };

                    let entity_service = entity_service.read();
                    entity_service.remove(entity_id).ok();
                }
            })
        });
}
