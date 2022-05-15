use crate::Scale2d;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_query::Write;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::math::matrix3::Matrix3;

pub fn scale_2d(entity_service: Const<EntityService>) {
    entity_service.for_each(
        entity_type!["Transform2d", "Scale2d"],
        Inject2::new(
            move |mut transform: Write<Transform2d>, scale: Read<Scale2d>| {
                transform.transform = transform.transform * Matrix3::scaling(scale.vec);
            },
        ),
    )
}
