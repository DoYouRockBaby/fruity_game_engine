use crate::Camera;
use crate::Transform2d;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;

pub fn draw_camera(
    graphic_service: Ref<dyn GraphicService>,
    query: Query<(With<Transform2d>, With<Camera>)>,
) {
    query.for_each(|(transform, camera)| {
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
            graphic_service.render_scene(view_proj, camera.background_color, camera.target.clone());
        }
    })
}
