use crate::components::rapier_collider::RapierCollider;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::circle_collider::CircleCollider;
use rapier2d::prelude::ColliderBuilder;
use rapier2d::prelude::SharedShape;
use rapier2d::prelude::*;

pub fn update_circle_collider(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        With<CircleCollider>,
        With<RapierCollider>,
    )>,
) {
    query.for_each(move |(transform, collider, rapier_collider)| {
        if let Some(rapier_collider) = rapier_collider.handle {
            let center = transform.transform * collider.center;
            let scaled_radius = transform.transform.scale() * collider.radius;

            // TODO: Use an ellipse collider
            let collider = ColliderBuilder::new(SharedShape::ball(f32::max(
                scaled_radius.x,
                scaled_radius.y,
            )))
            .translation(vector![center.x, center.y])
            .build();

            let mut rapier_2d_service = rapier_2d_service.write();
            let handle = rapier_2d_service.collider_set.insert(collider);
        }
    })
}
