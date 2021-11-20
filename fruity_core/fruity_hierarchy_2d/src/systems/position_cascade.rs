use crate::LocalPosition;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::EntityQueryWriteCallback3;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;

pub fn position_cascade(entity_service: Ref<EntityService>) {
    let entity_service_reader = entity_service.read();
    entity_service_reader.for_each_mut(
        entity_type!["LocalPosition", "Position", "Parent"],
        EntityQueryWriteCallback3::new(
            move |_entity_id: EntityId,
                  local_position: &mut LocalPosition,
                  position: &mut Position,
                  parent: &mut Parent| {
                let entity_service = entity_service.read();
                if let Some(parent_position) = get_parent_position(parent, &entity_service) {
                    position.pos = parent_position + local_position.pos;
                }
            },
        ),
    )
}

fn get_parent_position(parent: &Parent, entity_service: &EntityService) -> Option<Vector2d> {
    let parent_id = parent.parent_id.deref().map(|parent_id| parent_id)?;
    let parent_entity = entity_service.get_entity(parent_id)?;
    let parent_position = parent_entity.get_component("Position")?;
    let parent_position = parent_position.read();
    let parent_position = parent_position.as_any_ref().downcast_ref::<Position>()?;

    Some(parent_position.pos)
}
