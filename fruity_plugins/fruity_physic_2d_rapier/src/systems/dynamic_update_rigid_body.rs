use crate::DynamicRigidBody;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithOptionalMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic_2d::components::rotate_2d::Rotate2d;
use fruity_graphic_2d::components::translate_2d::Translate2d;
use rapier2d::prelude::*;

pub fn dynamic_update_rigid_body(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<DynamicRigidBody>,
        WithOptionalMut<Translate2d>,
        WithOptionalMut<Rotate2d>,
    )>,
) {
    query.for_each(move |(dynamic_rigid_body, translate_2d, rotate_2d)| {
        if let Some(rigid_body_handle) = dynamic_rigid_body.handle {
            let rapier_2d_service_reader = rapier_2d_service.read();

            if let Some(rapier_rigid_body) =
                rapier_2d_service_reader
                    .rigid_body_set
                    .get(RigidBodyHandle::from_raw_parts(
                        rigid_body_handle.0,
                        rigid_body_handle.1,
                    ))
            {
                if let Some(mut translate_2d) = translate_2d {
                    let translation = rapier_rigid_body.translation();
                    translate_2d.vec = Vector2d::new(translation.x, translation.y);
                }

                if let Some(mut rotate_2d) = rotate_2d {
                    let rotation = rapier_rigid_body.rotation();
                    rotate_2d.angle = rotation.angle();
                }
            }
        }
    })
}
