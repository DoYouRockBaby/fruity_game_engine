use crate::DynamicRigidBody;
use crate::Rapier2dService;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use rapier2d::prelude::*;

pub fn dynamic_update_rigid_body_prepare(
    rapier_2d_service: Ref<Rapier2dService>,
    query: Query<(With<Transform2d>, With<DynamicRigidBody>)>,
) {
    query.for_each(move |(transform, dynamic_rigid_body)| {
        if let Some(rigid_body_handle) = dynamic_rigid_body.handle {
            let mut rapier_2d_service_writer = rapier_2d_service.write();
            if let Some(rapier_rigid_body) =
                rapier_2d_service_writer
                    .rigid_body_set
                    .get_mut(RigidBodyHandle::from_raw_parts(
                        rigid_body_handle.0,
                        rigid_body_handle.1,
                    ))
            {
                let translation = transform.transform.translation();
                let rotation = transform.transform.rotation();

                rapier_rigid_body.set_gravity_scale(dynamic_rigid_body.gravity_scale, true);
                rapier_rigid_body.set_translation(vector![translation.x, translation.y], true);
                rapier_rigid_body.set_rotation(rotation, true);
            }
        }
    })
}
