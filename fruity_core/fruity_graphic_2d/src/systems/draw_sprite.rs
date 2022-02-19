use crate::Graphic2dService;
use crate::Sprite;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::entity::entity_query::Inject3;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::graphic_service::MaterialParam;
use maplit::hashmap;
use std::ops::Deref;

pub fn draw_sprite(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<Graphic2dService>,
) {
    entity_service.for_each(
        entity_type!["Transform2d", "Sprite"],
        Inject3::new(
            move |entity_id: EntityId, transform: &Transform2d, sprite: &Sprite| {
                let graphic_2d_service = graphic_2d_service.read();

                if let Some(material) = &sprite.material {
                    graphic_2d_service.draw_quad(entity_id, material.deref(),
                    hashmap! {
                        "transform".to_string() => MaterialParam::Matrix4(transform.transform.into()),
                    }, sprite.z_index);
                }
            },
        ),
    )
}
