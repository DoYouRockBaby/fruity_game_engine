use crate::gizmos_service::GizmosService;
use fruity_core::inject::Const;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::scale_2d::Scale2d;
use fruity_graphic_2d::components::transform_2d::Transform2d;
use fruity_graphic_2d::components::translate_2d::Translate2d;

pub fn draw_gizmos_2d(gizmos_service: Const<GizmosService>) {
    let inspector_state = use_global::<InspectorState>();

    if !inspector_state.is_gizmos_enabled() {
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
            transform
        } else {
            return;
        };

        if let Some(_) = entity_reader.read_typed_component::<Translate2d>("Translate2d") {
            let bottom_left = transform.transform * Vector2d::new(-0.5, -0.5);
            let top_right = transform.transform * Vector2d::new(0.5, 0.5);

            if let Some(_) = entity_reader.read_typed_component::<Scale2d>("Scale2d") {
                let selected_entity_2 = entity.clone();
                gizmos_service.draw_resize_helper(
                    bottom_left,
                    top_right,
                    Color::green(),
                    Color::red(),
                    move |fixed_x, fixed_y, drag_action| {
                        let selected_entity = selected_entity_2.clone();
                        let entity_reader = selected_entity.read();

                        // Get the translate and the scale origin
                        let translate_origin = {
                            let translate = entity_reader
                                .read_typed_component::<Translate2d>("Translate2d")
                                .unwrap();
                            translate.vec
                        };

                        let scale_origin = {
                            let scale = entity_reader
                                .read_typed_component::<Scale2d>("Scale2d")
                                .unwrap();
                            scale.vec
                        };

                        drag_action.while_dragging(move |cursor_position, start_pos| {
                            let cursor_movement = cursor_position - start_pos;
                            let entity_writer = entity.write();

                            // Move the entity with the cursor
                            let mut translate = entity_writer
                                .write_typed_component::<Translate2d>("Translate2d")
                                .unwrap();
                            translate.vec = translate_origin + cursor_movement / 2.0;

                            // Resize the entity with the cursor
                            let mut scale = entity_writer
                                .write_typed_component::<Scale2d>("Scale2d")
                                .unwrap();

                            scale.vec.x = if fixed_x {
                                scale_origin.x + cursor_movement.x
                            } else {
                                scale_origin.x - cursor_movement.x
                            };

                            scale.vec.y = if fixed_y {
                                scale_origin.y + cursor_movement.y
                            } else {
                                scale_origin.y - cursor_movement.y
                            };
                        });
                    },
                );
            }

            let center = (bottom_left + top_right) / 2.0;
            let size = top_right - bottom_left;
            gizmos_service.draw_move_helper(
                center,
                size,
                Color::green(),
                Color::red(),
                move |move_x, move_y, drag_action| {
                    let selected_entity = entity.clone();
                    let entity_reader = selected_entity.read();

                    // Get the translate origin
                    let translate_origin = {
                        let translate = entity_reader
                            .read_typed_component::<Translate2d>("Translate2d")
                            .unwrap();
                        translate.vec
                    };

                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let entity_writer = entity.write();
                        let mut translate = entity_writer
                            .write_typed_component::<Translate2d>("Translate2d")
                            .unwrap();

                        // Move the entity with the cursor
                        let cursor_movement = cursor_position - start_pos;
                        if move_x {
                            translate.vec.x = translate_origin.x + cursor_movement.x;
                        }

                        if move_y {
                            translate.vec.y = translate_origin.y + cursor_movement.y;
                        }
                    });
                },
            );
        }
    }
}
