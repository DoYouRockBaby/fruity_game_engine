use crate::ColliderState;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_editor::hooks::use_global;
use fruity_editor_graphic_2d::gizmos_service::DragAction;
use fruity_editor_graphic_2d::gizmos_service::GizmosService;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::input_service::InputService;
use fruity_physic_2d::components::circle_collider::CircleCollider;

pub fn draw_circle_collider_2d_gizmos(
    graphic_service: Ref<dyn GraphicService>,
    graphic_2d_service: Ref<Graphic2dService>,
    input_service: Ref<InputService>,
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

        if let Some(circle_collider) = collider.read_typed::<CircleCollider>() {
            let center = transform * circle_collider.center;
            let diff_center_extremity =
                transform * (circle_collider.center + Vector2d::new(0.0, -circle_collider.radius));
            let radius = (diff_center_extremity - center).length();
            let bottom = center + Vector2d::new(0.0, -radius);
            let size = Vector2d::new(radius, radius);

            // Draw the collider
            {
                let graphic_2d_service_reader = graphic_2d_service.read();
                graphic_2d_service_reader.draw_circle(
                    center,
                    radius,
                    4,
                    Color::overlay(),
                    Color::green(),
                    1000,
                );
            }

            // Draw the gizmos to move the center of the collider
            let collider_2 = collider.clone();
            gizmos_service.draw_move_helper(
                center,
                size,
                Color::green(),
                Color::red(),
                move |move_x, move_y, drag_action| {
                    let collider = collider_2.clone();

                    // Get the center origin
                    let center_origin = {
                        let circle_collider = collider.read_typed::<CircleCollider>().unwrap();
                        circle_collider.center
                    };

                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let mut circle_collider = collider.write_typed::<CircleCollider>().unwrap();

                        // Move the entity with the cursor
                        let cursor_movement = cursor_position - start_pos;
                        if move_x {
                            circle_collider.center.x = center_origin.x + cursor_movement.x;
                        }

                        if move_y {
                            circle_collider.center.y = center_origin.y + cursor_movement.y;
                        }
                    });
                },
            );

            // Get camera transform
            let camera_transform = {
                let graphic_service = graphic_service.read();
                graphic_service.get_camera_transform()
            };
            let camera_invert = camera_transform.invert();
            let radius_vec = camera_invert * Vector2d::new(0.012, 0.0);

            // Draw the gizmos to resize the radius of the collider
            if gizmos_service.draw_circle_helper(bottom, radius_vec.x, Color::green(), Color::red())
            {
                let collider_2 = collider.clone();
                DragAction::start(
                    move |drag_action| {
                        let collider = collider_2.clone();

                        // Get the radius origin
                        let radius_origin = {
                            let circle_collider = collider.read_typed::<CircleCollider>().unwrap();
                            circle_collider.radius
                        };

                        drag_action.while_dragging(move |cursor_position, start_pos| {
                            let mut circle_collider =
                                collider.write_typed::<CircleCollider>().unwrap();

                            // Resize the entity with the cursor
                            let cursor_movement = cursor_position - start_pos;
                            circle_collider.radius = radius_origin - cursor_movement.y;
                        });
                    },
                    input_service.clone(),
                    graphic_service.clone(),
                );
            }
        }
    }
}
