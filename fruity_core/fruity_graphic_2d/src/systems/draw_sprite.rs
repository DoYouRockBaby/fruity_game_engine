use crate::Graphic2dService;
use crate::Sprite;
use crate::Transform2d;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithId;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic::graphic_service::MaterialParam;
use maplit::hashmap;

pub fn draw_sprite(
    graphic_2d_service: Ref<Graphic2dService>,
    query: Query<(WithId, With<Transform2d>, With<Sprite>)>,
) {
    query.for_each(|(entity_id, transform, sprite)| {
        let graphic_2d_service = graphic_2d_service.read();

        if let Some(material) = &sprite.material {
            graphic_2d_service.draw_quad(
                entity_id,
                material.clone(),
                hashmap! {
                    "transform".to_string() => MaterialParam::Matrix4(transform.transform.into()),
                },
                sprite.z_index,
            );
        }
    })
}
