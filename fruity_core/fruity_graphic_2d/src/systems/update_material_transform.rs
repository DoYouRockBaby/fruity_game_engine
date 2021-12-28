use crate::Sprite;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;

pub fn update_material_transform(entity_service: Const<EntityService>) {
    entity_service.for_each(
        entity_type!["Transform2d", "Sprite"],
        Inject2::new(move |transform: &Transform2d, sprite: &Sprite| {
            if let Some(material) = &sprite.material {
                material.set_matrix4("transform", transform.transform.into());
            }
        }),
    )
}
