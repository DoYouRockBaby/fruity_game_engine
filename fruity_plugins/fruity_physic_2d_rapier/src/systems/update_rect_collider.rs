use crate::components::rapier_rect_collider::RapierRectCollider;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithExtension;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::rect_collider::RectCollider;
use rapier2d::prelude::ColliderHandle;
use rapier2d::prelude::*;

pub fn update_rect_collider(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        WithExtension<RectCollider, RapierRectCollider>,
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
                let scale = transform.transform.scale();
                let bottom_left = transform.transform * collider.bottom_left;
                let top_right = transform.transform * collider.top_right;

                let translate = (bottom_left + top_right) / 2.0;
                let rotation = transform.transform.rotation();
                let size = Vector2d::new(
                    scale.x * (collider.top_right.x - collider.bottom_left.x) / 2.0,
                    scale.y * (collider.top_right.y - collider.bottom_left.y) / 2.0,
                );

                rapier_collider.set_shape(SharedShape::cuboid(size.x, size.y));
                rapier_collider.set_translation(vector![translate.x, translate.y]);
                rapier_collider.set_rotation(rotation);
            }
        }
    })
}
