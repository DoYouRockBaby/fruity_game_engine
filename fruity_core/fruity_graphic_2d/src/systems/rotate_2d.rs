use crate::Rotate2d;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::math::matrix3::Matrix3;

pub fn rotate_2d(entity_service: Const<EntityService>) {
    entity_service.for_each(
        entity_type!["Transform2d", "Rotate2d"],
        Inject2::new(move |mut transform: &mut Transform2d, rotate: &Rotate2d| {
            transform.transform = transform.transform * Matrix3::rotation(rotate.angle);
        }),
    )
}
