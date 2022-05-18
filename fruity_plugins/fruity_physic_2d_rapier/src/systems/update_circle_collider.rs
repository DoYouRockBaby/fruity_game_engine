use crate::components::rapier_collider::RapierCollider;
use crate::Rapier2dService;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject3;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::circle_collider::CircleCollider;
use rapier2d::prelude::ColliderBuilder;
use rapier2d::prelude::SharedShape;
use rapier2d::prelude::*;

pub fn update_circle_collider(
    entity_service: Const<EntityService>,
    rapier_2d_service: Ref<Rapier2dService>,
) {
    entity_service.for_each(
        entity_type!["CircleCollider"],
        Inject3::new(
            move |transform: Read<Transform2d>,
                  collider: Read<CircleCollider>,
                  rapier_collider: Read<RapierCollider>| {
                if let Some(rapier_collider) = rapier_collider.handle {
                    let center = transform.transform * collider.center;
                    let scaled_radius = transform.transform.scale() * collider.radius;

                    let collider = ColliderBuilder::new(SharedShape::ball(f32::max(
                        scaled_radius.x,
                        scaled_radius.y,
                    )))
                    .translation(vector![center.x, center.y])
                    .build();

                    let mut rapier_2d_service = rapier_2d_service.write();
                    let handle = rapier_2d_service.collider_set.insert(collider);
                }
            },
        ),
    )
}
