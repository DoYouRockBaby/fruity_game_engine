use crate::components::rapier_rect_collider::RapierRectCollider;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithExtensionMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::system::system_service::StartupDisposeSystemCallback;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::rect_collider::RectCollider;
use rapier2d::prelude::ColliderBuilder;
use rapier2d::prelude::SharedShape;
use rapier2d::prelude::*;

pub fn initialize_rect_collider(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        WithExtensionMut<RectCollider, RapierRectCollider>,
    )>,
) -> StartupDisposeSystemCallback {
    let handle = query.on_created(move |(transform, (collider, mut rapier_collider))| {
        let scale = transform.transform.scale();
        let bottom_left = transform.transform * collider.bottom_left;
        let top_right = transform.transform * collider.top_right;

        let translate = (bottom_left + top_right) / 2.0;
        let rotation = transform.transform.rotation();
        let size = Vector2d::new(
            scale.x * (top_right.x - bottom_left.x),
            scale.y * (top_right.y - bottom_left.y),
        );

        let collider = ColliderBuilder::new(SharedShape::cuboid(size.x, size.y))
            .translation(vector![translate.x, translate.y])
            .rotation(rotation)
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
