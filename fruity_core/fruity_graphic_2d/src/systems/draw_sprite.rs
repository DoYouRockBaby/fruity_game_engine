use crate::Graphic2dService;
use crate::Position;
use crate::Size;
use crate::Sprite;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::EntityQueryReadCallback3;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;

pub fn draw_sprite(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<dyn Graphic2dService>,
) {
    entity_service.for_each(
        entity_type!["Position", "Size", "Sprite"],
        EntityQueryReadCallback3::new(move |position: &Position, size: &Size, sprite: &Sprite| {
            if let Some(material) = &sprite.material.0 {
                let graphic_2d_service = graphic_2d_service.read();

                graphic_2d_service.draw_square(
                    position.pos,
                    size.size,
                    sprite.z_index,
                    material.clone(),
                );
            }
        }),
    )
}
