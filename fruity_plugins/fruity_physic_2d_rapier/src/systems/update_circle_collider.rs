use crate::components::rapier_circle_collider::RapierCircleCollider;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithExtension;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::circle_collider::CircleCollider;
use rapier2d::prelude::ColliderHandle;
use rapier2d::prelude::SharedShape;
use rapier2d::prelude::*;

pub fn update_circle_collider(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        WithExtension<CircleCollider, RapierCircleCollider>,
    )>,
) {
    query.for_each(move |(transform, (collider, rapier_collider))| {
        if let Some(collider_handle) = rapier_collider.handle {
            let mut rapier_2d_service_writer = rapier_2d_service.write();
            if let Some(rapier_collider) =
                rapier_2d_service_writer
                    .collider_set
                    .get_mut(ColliderHandle::from_raw_parts(
                        collider_handle.0,
                        collider_handle.1,
                    ))
            {
                let center = transform.transform * collider.center;
                let scaled_radius = transform.transform.scale() * collider.radius;

                rapier_collider.set_shape(SharedShape::ball(f32::max(
                    scaled_radius.x,
                    scaled_radius.y,
                )));
                rapier_collider.set_translation(vector![center.x, center.y]);
            }
        }
    })
}
