use crate::Scale2d;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::EntityQueryWriteCallback2;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::math::matrix3::Matrix3;

pub fn scale_2d(entity_service: Const<EntityService>) {
    entity_service.for_each_mut(
        entity_type!["Transform2d", "Scale2d"],
        EntityQueryWriteCallback2::new(
            move |_entity_id: EntityId, transform: &mut Transform2d, scale: &mut Scale2d| {
                transform.transform = transform.transform * Matrix3::scaling(scale.vec);
            },
        ),
    )
}
