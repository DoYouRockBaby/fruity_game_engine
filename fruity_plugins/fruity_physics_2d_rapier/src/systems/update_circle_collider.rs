use crate::Rapier2dService;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject4;
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
        entity_type!["CircleCollider", "?Translate2d", "?Rotate2d", "?Scale2d"],
        Inject4::new(
            move |collider: &CircleCollider,
                  translate: Option<&Translate2d>,
                  rotate: Option<&Rotate2d>,
                  scale: Option<&Scale2d>| {
                let rapier_2d_service = rapier_2d_service.read();
            },
        ),
    )
}
