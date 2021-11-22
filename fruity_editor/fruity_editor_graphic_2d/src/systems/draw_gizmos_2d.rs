use crate::gizmos_service::GizmosService;
use fruity_core::inject::Const;
use fruity_ecs::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::scale_2d::Scale2d;
use fruity_graphic_2d::components::translate_2d::Translate2d;

pub fn draw_gizmos_2d(gizmos_service: Const<GizmosService>) {
    let inspector_state = use_global::<InspectorState>();

    if let Some(selected) = inspector_state.get_selected() {
        let entity =
            if let Some(entity) = selected.as_any_ref().downcast_ref::<EntitySharedRwLock>() {
                entity
            } else {
                return;
            };

        // Get the selected entity bounds
        let (bottom_left, top_right) = {
            let entity_reader = entity.read();
            let translate = if let Some(translate) =
                entity_reader.get_component::<Translate2d>("Translate2d")
            {
                translate
            } else {
                return;
            };

            let scale = if let Some(scale) = entity_reader.get_component::<Scale2d>("Scale2d") {
                scale
            } else {
                return;
            };

            let bottom_left = translate.vec;
            let top_right = translate.vec + scale.vec;

            (bottom_left, top_right)
        };

        // Draw the resize helper
        let selected_entity_2 = entity.clone();
        gizmos_service.draw_resize_helper(
            bottom_left,
            top_right,
            GREEN,
            RED,
            move |move_x, move_y, drag_action| {
                let selected_entity = entity.clone();

                // Get the translate origin
                let translate_origin = {
                    let entity_reader = selected_entity.read();
                    let translate = entity_reader
                        .get_component::<Translate2d>("Translate2d")
                        .unwrap();
                    translate.vec
                };

                drag_action.while_dragging(move |cursor_position, start_pos| {
                    let mut entity_writer = selected_entity.write();
                    let translate = entity_writer
                        .get_component_mut::<Translate2d>("Translate2d")
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
            move |fixed_x, fixed_y, drag_action| {
                let selected_entity = selected_entity_2.clone();

                // Get the translate and the scale origin
                let (translate_origin, scale_origin) = {
                    let entity_reader = selected_entity.read();
                    let translate = entity_reader
                        .get_component::<Translate2d>("Translate2d")
                        .unwrap();
                    let scale = entity_reader.get_component::<Scale2d>("Scale2d").unwrap();

                    (translate.vec, scale.vec)
                };

                drag_action.while_dragging(move |cursor_position, start_pos| {
                    let mut entity_writer = selected_entity.write();

                    let cursor_movement = cursor_position - start_pos;

                    // Move the entity with the cursor
                    let translate = entity_writer
                        .get_component_mut::<Translate2d>("Translate2d")
                        .unwrap();

                    translate.vec.x = if fixed_x {
                        translate_origin.x
                    } else {
                        translate_origin.x + cursor_movement.x
                    };

                    translate.vec.y = if fixed_y {
                        translate_origin.y
                    } else {
                        translate_origin.y + cursor_movement.y
                    };

                    // Resize the entity with the cursor
                    let scale = entity_writer
                        .get_component_mut::<Scale2d>("Scale2d")
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
}
