use crate::Camera;
use crate::Graphic2dService;
use crate::Transform2d;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::Inject2;
use fruity_ecs::entity::entity_query::Read;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_ecs::system::system_service::SystemService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;

pub fn draw_camera(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<dyn Graphic2dService>,
    system_service: Ref<SystemService>,
) {
    entity_service.for_each(
        entity_type!["Transform2d", "Camera"],
        Inject2::new(move |transform: Read<Transform2d>, camera: Read<Camera>| {
            let bottom_left = transform.transform * Vector2d::new(0.0, 0.0);
            let top_right = transform.transform * Vector2d::new(1.0, 1.0);

            let view_proj = Matrix4::from_rect(
                bottom_left.x,
                top_right.x,
                bottom_left.y,
                top_right.y,
                camera.near,
                camera.far,
            );

            // Start the pass
            {
                let graphic_2d_service = graphic_2d_service.read();
                graphic_2d_service.start_pass(view_proj);
            }

            // Render the draw system pool and avoir the normal system treatment
            {
                let system_service = system_service.read();
                system_service.ignore_pool_once(&98);
                system_service.run_pool(&98);
            }

            // End the pass
            {
                let graphic_2d_service = graphic_2d_service.read();
                graphic_2d_service.end_pass();
            }
        }),
    )
}
