use crate::LocalPosition;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::EntityQueryWriteCallback3;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic_2d::components::position::Position;
use fruity_hierarchy::components::parent::Parent;

pub fn size_cascade(entity_service: Const<EntityService>) {
    entity_service.for_each_mut(
        entity_type!["LocalSize", "Size", "Parent"],
        EntityQueryWriteCallback3::new(
            move |_entity_id: EntityId,
                  local_position: &mut LocalPosition,
                  position: &mut Position,
                  parent: &mut Parent| {},
        ),
    )
}
