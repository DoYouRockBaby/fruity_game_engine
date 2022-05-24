use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

pub fn transform_2d_cascade(
    entity_service: Ref<EntityService>,
    query: Query<(With<Parent>, WithMut<Transform2d>)>,
) {
    let mut current_nested_level = 1;
    while transform_2d_cascade_for_nested_level(
        entity_service.clone(),
        query.clone(),
        current_nested_level,
    ) {
        current_nested_level += 1;
    }
}

pub fn transform_2d_cascade_for_nested_level(
    entity_service: Ref<EntityService>,
    query: Query<(With<Parent>, WithMut<Transform2d>)>,
    nested_level: usize,
) -> bool {
    let did_transform = Arc::new(AtomicBool::new(false));
    let did_transform_2 = did_transform.clone();

    query.for_each(move |(child, mut transform)| {
        if child.nested_level == nested_level {
            // Get the parent entity reference
            let parent_entity = if let Some(parent_id) = &child.parent_id.deref() {
                let entity_service_reader = entity_service.read();
                entity_service_reader.get_entity(*parent_id)
            } else {
                None
            };

            // Apply the parent transform to the child
            if let Some(parent_entity) = parent_entity {
                if let Some(parent_transform) =
                    parent_entity.read().read_single_component::<Transform2d>()
                {
                    transform.transform = parent_transform.transform * transform.transform;
                    did_transform.store(true, Relaxed);
                }
            }
        }
    });

    did_transform_2.load(Relaxed)
}
