use crate::Camera;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;

pub fn draw_camera(entity_service: Const<EntityService>, graphic_service: Ref<dyn GraphicService>) {
    entity_service.for_each(
        entity_type!["Transform2d", "Camera"],
        Inject2::new(move |transform: Read<Transform2d>, camera: Read<Camera>| {
            let bottom_left = transform.transform * Vector2d::new(-0.5, -0.5);
            let top_right = transform.transform * Vector2d::new(0.5, 0.5);

            let view_proj = Matrix4::from_rect(
                bottom_left.x,
                top_right.x,
                bottom_left.y,
                top_right.y,
                camera.near,
                camera.far,
            );

            // Render the scene
            {
                puffin::profile_scope!("render_scene");
                let graphic_service = graphic_service.read();
                graphic_service.render_scene(
                    view_proj,
                    camera.background_color,
                    camera.target.clone(),
                );
            }
        }),
    )
}
