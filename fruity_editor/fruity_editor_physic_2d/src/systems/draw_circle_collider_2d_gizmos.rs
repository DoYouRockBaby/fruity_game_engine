use crate::ColliderState;
use fruity_core::convert::FruityInto;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_editor::mutations::mutation_service::MutationService;
use fruity_editor::mutations::set_field_mutation::SetFieldMutation;
use fruity_editor_graphic_2d::gizmos_service::GizmosService;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::drag_service::DragService;
use fruity_physic_2d::components::circle_collider::CircleCollider;

pub fn draw_circle_collider_2d_gizmos(
    collider_state: Const<ColliderState>,
    gizmos_service: Const<GizmosService>,
    graphic_service: Ref<dyn GraphicService>,
    graphic_2d_service: Ref<Graphic2dService>,
    drag_service: Ref<DragService>,
    mutation_service: Ref<MutationService>,
) {
    if !collider_state.is_editing_collider() {
        return;
    }

    if let Some(collider) = collider_state.get_editing_collider() {
        let transform = {
            let entity_reader = collider.read_entity();

            if let Some(transform) = entity_reader
                .read_single_component::<Transform2d>()
                .map(|transform| transform.transform)
            {
                transform
            } else {
                return;
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
            let mutation_service_2 = mutation_service.clone();
            gizmos_service.draw_move_helper(
                center,
                size,
                Color::green(),
                Color::red(),
                |move_x, move_y| {
                    let graphic_service = graphic_service.clone();
                    let collider = collider_2.clone();
                    let collider_2 = collider_2.clone();

                    // Get the center origin
                    let center_origin = {
                        let circle_collider = collider.read_typed::<CircleCollider>().unwrap();
                        circle_collider.center
                    };

                    let mutation_service = mutation_service.clone();
                    (
                        Box::new(move |action| {
                            let (cursor_pos, start_pos) = {
                                let graphic_service_reader = graphic_service.read();
                                (
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.cursor_pos.0,
                                        action.cursor_pos.1,
                                    ),
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.start_pos.0,
                                        action.start_pos.1,
                                    ),
                                )
                            };

                            let mut circle_collider =
                                collider.write_typed::<CircleCollider>().unwrap();

                            // Move the entity with the cursor
                            let cursor_movement = cursor_pos - start_pos;
                            if move_x {
                                circle_collider.center.x = center_origin.x + cursor_movement.x;
                            }

                            if move_y {
                                circle_collider.center.y = center_origin.y + cursor_movement.y;
                            }
                        }),
                        Box::new(move |_| {
                            let collider = collider_2.clone();

                            let mut mutation_service = mutation_service.write();

                            // Get current values
                            let center_current = {
                                let circle_collider =
                                    collider.read_typed::<CircleCollider>().unwrap();
                                circle_collider.center
                            };

                            // Store the mutations
                            mutation_service.push_action(SetFieldMutation {
                                target: Box::new(collider.clone()),
                                field: "center".to_string(),
                                previous_value: center_origin.fruity_into(),
                                new_value: center_current.fruity_into(),
                            });
                        }),
                    )
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
                let drag_service_reader = drag_service.read();
                drag_service_reader.start_drag(move || {
                    let collider = collider_2.clone();
                    let collider_2 = collider_2.clone();

                    // Get the radius origin
                    let radius_origin = {
                        let circle_collider = collider.read_typed::<CircleCollider>().unwrap();
                        circle_collider.radius
                    };

                    let graphic_service = graphic_service.clone();
                    let mutation_service = mutation_service_2.clone();
                    (
                        Box::new(move |action| {
                            let collider = collider.clone();

                            let (cursor_pos, start_pos) = {
                                let graphic_service_reader = graphic_service.read();
                                (
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.cursor_pos.0,
                                        action.cursor_pos.1,
                                    ),
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.start_pos.0,
                                        action.start_pos.1,
                                    ),
                                )
                            };

                            let mut circle_collider =
                                collider.write_typed::<CircleCollider>().unwrap();

                            // Resize the entity with the cursor
                            let cursor_movement = cursor_pos - start_pos;
                            circle_collider.radius = radius_origin - cursor_movement.y;
                        }),
                        Box::new(move |_| {
                            let collider = collider_2.clone();

                            let mut mutation_service = mutation_service.write();

                            // Get current values
                            let radius_current = {
                                let circle_collider =
                                    collider.read_typed::<CircleCollider>().unwrap();
                                circle_collider.radius
                            };

                            // Store the mutations
                            mutation_service.push_action(SetFieldMutation {
                                target: Box::new(collider.clone()),
                                field: "center".to_string(),
                                previous_value: radius_origin.fruity_into(),
                                new_value: radius_current.fruity_into(),
                            });
                        }),
                    )
                });
            }
        }
    }
}
