use crate::Graphic2dService;
use crate::Sprite;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;

pub fn draw_sprite(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<dyn Graphic2dService>,
) {
    entity_service.for_each(
        entity_type!["Transform2d", "Sprite"],
        Inject2::new(move |transform: Read<Transform2d>, sprite: Read<Sprite>| {
            if let Some(material) = &sprite.material {
                let graphic_2d_service = graphic_2d_service.read();

                graphic_2d_service.draw_square(
                    transform.transform,
                    sprite.z_index,
                    material.clone(),
                );
            }
        }),
    )
}
