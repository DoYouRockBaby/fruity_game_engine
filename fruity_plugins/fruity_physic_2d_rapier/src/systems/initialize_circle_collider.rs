use crate::components::rapier_circle_collider::RapierCircleCollider;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithExtensionMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::system::system_service::StartupDisposeSystemCallback;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::circle_collider::CircleCollider;
use rapier2d::prelude::ColliderBuilder;
use rapier2d::prelude::SharedShape;
use rapier2d::prelude::*;

pub fn initialize_circle_collider(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        WithExtensionMut<CircleCollider, RapierCircleCollider>,
    )>,
) -> StartupDisposeSystemCallback {
    let handle = query.on_created(move |(transform, (collider, mut rapier_collider))| {
        let center = transform.transform * collider.center;
        let scaled_radius = transform.transform.scale() * collider.radius;

        // TODO: Use an ellipse collider
        let collider = ColliderBuilder::new(SharedShape::ball(f32::max(
            scaled_radius.x,
            scaled_radius.y,
        )))
        .translation(vector![center.x, center.y])
        .build();

        let collider_handle = {
            let mut rapier_2d_service = rapier_2d_service.write();
            rapier_2d_service.collider_set.insert(collider)
        };

        rapier_collider.handle = Some(collider_handle.into_raw_parts());

        let rapier_2d_service = rapier_2d_service.clone();
        Some(Box::new(move || {
            let mut rapier_2d_service = rapier_2d_service.write();
            rapier_2d_service.remove_collider(collider_handle);
        }))
    });

    Some(Box::new(move || {
        handle.dispose_by_ref();
    }))
}
