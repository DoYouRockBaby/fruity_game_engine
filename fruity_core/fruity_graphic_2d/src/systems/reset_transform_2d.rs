use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity_query::Inject1;
use fruity_ecs::entity::entity_query::Write;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::math::matrix3::Matrix3;

pub fn reset_transform_2d(entity_service: Const<EntityService>) {
    entity_service.for_each(
        entity_type!["Transform2d"],
        Inject1::new(|mut transform: Write<Transform2d>| {
            transform.transform = Matrix3::new_identity();
        }),
    )
}
