use crate::Graphic2dService;
use crate::Sprite;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject1;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use std::ops::Deref;

pub fn draw_sprite(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<Graphic2dService>,
) {
    entity_service.for_each(
        entity_type!["Sprite"],
        Inject1::new(move |sprite: &Sprite| {
            let graphic_2d_service = graphic_2d_service.read();

            if let Some(material) = &sprite.material {
                graphic_2d_service.draw_quad(material.deref(), sprite.z_index);
            }
        }),
    )
}
