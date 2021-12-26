use crate::Parent;
use fruity_core::inject::Ref;
use fruity_ecs::component::component_reference::ComponentReference;
use fruity_ecs::entity::entity_query::Inject1;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::ops::Deref;
use std::ops::DerefMut;

pub fn update_nested_level(entity_service: Ref<EntityService>) {
    let entity_service_reader = entity_service.read();
    entity_service_reader.for_each(
        entity_type!["Parent"],
        Inject1::new(move |child: ComponentReference| {
            let mut child_writer = child.write();
            let mut child_writer = child_writer
                .deref_mut()
                .as_any_mut()
                .downcast_mut::<Parent>()
                .unwrap();

            // Get the parent entity reference
            let parent_entity = if let Some(parent_id) = &child_writer.parent_id.deref() {
                let entity_service_reader = entity_service.read();
                entity_service_reader.get_entity(*parent_id)
            } else {
                None
            };

            // Set the nested level as the parent one plus one
            if let Some(parent_entity) = parent_entity {
                if let Some(parent) = parent_entity.read_component::<Parent>("Parent") {
                    child_writer.nested_level = parent.nested_level + 1;
                } else {
                    child_writer.nested_level = 1;
                }
            }

            // When parent is updated, we update the nested level
            let child_2 = child.clone();
            let entity_service = entity_service.clone();
            child_writer
                .parent_id
                .on_updated
                .add_observer(move |parent_id| {
                    let mut child_writer = child_2.write();
                    let mut child_writer = child_writer
                        .deref_mut()
                        .as_any_mut()
                        .downcast_mut::<Parent>()
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
                        if let Some(parent) = parent_entity.read_component::<Parent>("Parent") {
                            child_writer.nested_level = parent.nested_level + 1;
                        } else {
                            child_writer.nested_level = 1;
                        }
                    }
                });
        }),
    )
}
