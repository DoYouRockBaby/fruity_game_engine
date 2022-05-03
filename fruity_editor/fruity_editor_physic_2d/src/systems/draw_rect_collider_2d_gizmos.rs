use crate::ColliderState;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_editor::hooks::use_global;
use fruity_editor_graphic_2d::gizmos_service::GizmosService;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_physic_2d::components::rect_collider::RectCollider;

pub fn draw_rectangle_collider_2d_gizmos(
    graphic_2d_service: Ref<Graphic2dService>,
    gizmos_service: Const<GizmosService>,
) {
    let collider_state = use_global::<ColliderState>();

    if !collider_state.is_editing_collider() {
        return;
    }

    if let Some(collider) = collider_state.get_editing_collider() {
        let transform = {
            let entity_reader = collider.read_entity();

            if let Some(transform) =
                entity_reader.read_single_typed_component::<Transform2d>("Transform2d")
            {
                transform.transform.clone()
            } else {
                Matrix3::default()
            }
        };

        if let Some(rect_collider) = collider.clone().read_typed::<RectCollider>() {
            let bottom_left = transform * rect_collider.bottom_left;
            let top_right = transform * rect_collider.top_right;

            // Draw the collider
            {
                let graphic_2d_service_reader = graphic_2d_service.read();
                graphic_2d_service_reader.draw_rect(
                    bottom_left,
                    top_right,
                    4,
                    Color::overlay(),
                    Color::green(),
                    1000,
                );
            }

            // Draw the gizmos to resize the collider
            gizmos_service.draw_resize_helper(
                bottom_left,
                top_right,
                Color::green(),
                Color::red(),
                move |fixed_x, fixed_y, drag_action| {
                    let collider = collider.clone();

                    // Get the rect origin
                    let (bottom_left_origin, top_right_origin) = {
                        let collider = collider.read_typed::<RectCollider>().unwrap();
                        (collider.bottom_left, collider.top_right)
                    };

                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let cursor_movement = cursor_position - start_pos;

                        // Move the entity with the cursor
                        let mut collider = collider.write_typed::<RectCollider>().unwrap();
                        collider.bottom_left = bottom_left_origin + cursor_movement / 2.0;

                        // Resize the entity with the cursor
                        collider.top_right.x = if fixed_x {
                            top_right_origin.x + cursor_movement.x
                        } else {
                            top_right_origin.x - cursor_movement.x
                        };

                        collider.top_right.y = if fixed_y {
                            top_right_origin.y + cursor_movement.y
                        } else {
                            top_right_origin.y - cursor_movement.y
                        };
                    });
                },
            );
        }
    }
}
