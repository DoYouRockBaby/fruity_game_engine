use crate::Camera;
use crate::Graphic2dService;
use crate::Position;
use crate::Size;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_query::EntityQueryReadCallback3;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity_type;
use fruity_ecs::system::system_service::SystemService;
use fruity_graphic::math::Matrix4;

pub fn draw_camera(
    entity_service: Const<EntityService>,
    graphic_2d_service: Ref<dyn Graphic2dService>,
    system_service: Ref<SystemService>,
) {
    entity_service.for_each(
        entity_type!["Position", "Size", "Camera"],
        EntityQueryReadCallback3::new(move |position: &Position, size: &Size, camera: &Camera| {
            let view_proj = Matrix4::from_rect(
                position.pos.x,
                position.pos.x + size.size.x,
                position.pos.y,
                position.pos.y + size.size.y,
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
