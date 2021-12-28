use crate::ColliderState;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor_graphic_2d::gizmos_service::DragAction;
use fruity_editor_graphic_2d::gizmos_service::GizmosService;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::OVERLAY;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::input_service::InputService;
use fruity_physic_2d::components::circle_collider::CircleCollider;

pub fn draw_circle_collider_2d_gizmos(
    graphic_2d_service: Ref<Graphic2dService>,
    input_service: Ref<InputService>,
    gizmos_service: Const<GizmosService>,
) {
    let inspector_state = use_global::<InspectorState>();
    let collider_state = use_global::<ColliderState>();

    if !collider_state.is_editing_collider() {
        return;
    }

    if let Some(selected) = inspector_state.get_selected() {
        let entity = if let Some(entity) = selected.as_any_ref().downcast_ref::<EntityReference>() {
            entity
        } else {
            return;
        };
        let entity_reader = entity.read();

        let transform = if let Some(transform) =
            entity_reader.read_typed_component::<Transform2d>("Transform2d")
        {
            transform.transform.clone()
        } else {
            Matrix3::default()
        };

        if let Some(circle_collider) =
            entity_reader.read_typed_component::<CircleCollider>("CircleCollider")
        {
            let center = transform * circle_collider.center;
            let diff_center_extremity =
                transform * (circle_collider.center + Vector2d::new(0.0, -circle_collider.radius));
            let radius = (diff_center_extremity - center).length();
            let bottom = center + Vector2d::new(0.0, -radius);
            let size = Vector2d::new(radius, radius);

            // Draw the collider
            {
                let graphic_2d_service_reader = graphic_2d_service.read();
                graphic_2d_service_reader.draw_circle(center, radius, 4, OVERLAY, GREEN, 1000);
            }

            // Draw the gizmos to move the center of the collider
            gizmos_service.draw_move_helper(
                center,
                size,
                GREEN,
                RED,
                move |move_x, move_y, drag_action| {
                    let selected_entity = entity.clone();
                    let entity_reader = selected_entity.read();

                    // Get the center origin
                    let center_origin = {
                        let circle_collider = entity_reader
                            .read_typed_component::<CircleCollider>("CircleCollider")
                            .unwrap();
                        circle_collider.center
                    };

                    let selected_entity = entity.clone();
                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let entity_writer = selected_entity.write();
                        let mut circle_collider = entity_writer
                            .write_typed_component::<CircleCollider>("CircleCollider")
                            .unwrap();

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

            // Draw the gizmos to resize the radius of the collider
            if gizmos_service.draw_circle_helper(bottom, 0.012, GREEN, RED) {
                DragAction::start(
                    move |drag_action| {
                        let selected_entity = entity.clone();
                        let entity_reader = selected_entity.read();

                        // Get the radius origin
                        let radius_origin = {
                            let circle_collider = entity_reader
                                .read_typed_component::<CircleCollider>("CircleCollider")
                                .unwrap();
                            circle_collider.radius
                        };

                        drag_action.while_dragging(move |cursor_position, start_pos| {
                            let entity_writer = entity.write();
                            let mut circle_collider = entity_writer
                                .write_typed_component::<CircleCollider>("CircleCollider")
                                .unwrap();

                            // Resize the entity with the cursor
                            let cursor_movement = cursor_position - start_pos;
                            circle_collider.radius = radius_origin - cursor_movement.y;
                        });
                    },
                    input_service.clone(),
                    graphic_2d_service.clone(),
                );
            }
        }
    }
}
