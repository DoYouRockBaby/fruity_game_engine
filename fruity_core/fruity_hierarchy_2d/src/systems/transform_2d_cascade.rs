use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_query::Write;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

pub fn transform_2d_cascade(entity_service: Ref<EntityService>) {
    let mut current_nested_level = 1;
    while transform_2d_cascade_for_nested_level(entity_service.clone(), current_nested_level) {
        current_nested_level += 1;
    }
}

pub fn transform_2d_cascade_for_nested_level(
    entity_service: Ref<EntityService>,
    nested_level: usize,
) -> bool {
    let did_transform = Arc::new(AtomicBool::new(false));
    let did_transform_2 = did_transform.clone();

    let entity_service_reader = entity_service.read();
    entity_service_reader.for_each(
        entity_type!["Parent", "Transform2d"],
        Inject2::new(
            move |child: Read<Parent>, mut transform: Write<Transform2d>| {
                if child.nested_level == nested_level {
                    // Get the parent entity reference
                    let parent_components = if let Some(parent_id) = &child.parent_id.deref() {
                        let entity_service_reader = entity_service.read();
                        entity_service_reader.get_entity(*parent_id, entity_type!["Transform2d"])
                    } else {
                        None
                    };

                    // Apply the parent transform to the child
                    if let Some(parent_components) = parent_components {
                        if parent_components.len() == 1 {
                            let parent_transform = parent_components.get(0).unwrap().read();
                            let parent_transform = parent_transform
                                .as_any_ref()
                                .downcast_ref::<Transform2d>()
                                .unwrap();

                            transform.transform = parent_transform.transform * transform.transform;
                            did_transform.store(true, Relaxed);
                        }
                    }
                }
            },
        ),
    );

    did_transform_2.load(Relaxed)
}
