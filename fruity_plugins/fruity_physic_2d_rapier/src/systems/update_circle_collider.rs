use crate::Rapier2dService;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject1;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic_2d::components::rotate_2d::Rotate2d;
use fruity_graphic_2d::components::scale_2d::Scale2d;
use fruity_graphic_2d::components::translate_2d::Translate2d;
use fruity_physic_2d::components::circle_collider::CircleCollider;

pub fn update_circle_collider(
    entity_service: Const<EntityService>,
    rapier_2d_service: Ref<Rapier2dService>,
) {
    entity_service.for_each(
        entity_type!["CircleCollider"],
        Inject1::new(move |collider: Read<CircleCollider>| {
            let rapier_2d_service = rapier_2d_service.read();
        }),
    )
}
