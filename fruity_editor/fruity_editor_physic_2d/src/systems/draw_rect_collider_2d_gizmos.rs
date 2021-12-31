use crate::ColliderState;
use fruity_core::inject::Const;
use fruity_core::inject::Ref;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
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

        if let Some(rect_collider) =
            entity_reader.read_typed_component::<RectCollider>("RectCollider")
        {
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
                    let selected_entity = entity.clone();
                    let entity_reader = selected_entity.read();

                    // Get the rect origin
                    let (bottom_left_origin, top_right_origin) = {
                        let collider = entity_reader
                            .read_typed_component::<RectCollider>("RectCollider")
                            .unwrap();
                        (collider.bottom_left, collider.top_right)
                    };

                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let entity_writer = entity.write();
                        let cursor_movement = cursor_position - start_pos;

                        // Move the entity with the cursor
                        let mut collider = entity_writer
                            .write_typed_component::<RectCollider>("RectCollider")
                            .unwrap();
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
