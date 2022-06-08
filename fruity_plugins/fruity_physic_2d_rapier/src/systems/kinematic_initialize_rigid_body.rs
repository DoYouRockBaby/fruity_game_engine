use crate::KinematicRigidBody;
use crate::Rapier2dService;
use crate::RapierRectCollider;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithExtension;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::system::system_service::StartupDisposeSystemCallback;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_physic_2d::components::rect_collider::RectCollider;
use rapier2d::prelude::*;

pub fn kinematic_initialize_rigid_body(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(
        With<Transform2d>,
        WithMut<KinematicRigidBody>,
        WithExtension<RectCollider, RapierRectCollider>,
    )>,
) -> StartupDisposeSystemCallback {
    let handle = query.on_created(
        move |(transform, mut kinematic_rigid_body, (_, rapier_collider))| {
            let translation = transform.transform.translation();
            let rotation = transform.transform.rotation();

            let rigid_body = RigidBodyBuilder::kinematic_position_based()
                .translation(vector![translation.x, translation.y])
                .rotation(rotation)
                .build();

            let rigid_body_handle = {
                let mut rapier_2d_service = rapier_2d_service.write();
                rapier_2d_service.rigid_body_set.insert(rigid_body)
            };

            kinematic_rigid_body.handle = Some(rigid_body_handle.into_raw_parts());

            if let Some(collider_handle) = rapier_collider.handle {
                let mut rapier_2d_service = rapier_2d_service.write();

                rapier_2d_service.set_collider_parent(
                    ColliderHandle::from_raw_parts(collider_handle.0, collider_handle.1),
                    rigid_body_handle,
                );
            }

            let rapier_2d_service = rapier_2d_service.clone();
            Some(Box::new(move || {
                let mut rapier_2d_service = rapier_2d_service.write();
                rapier_2d_service.remove_rigid_body(rigid_body_handle);
            }))
        },
    );

    Some(Box::new(move || {
        handle.dispose_by_ref();
    }))
}
