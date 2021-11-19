use crate::components::parent::Parent;
use std::cell::Ref;

pub fn delete_cascade(parent: Ref<Parent>) {
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
