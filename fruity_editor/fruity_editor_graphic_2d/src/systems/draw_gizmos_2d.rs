use crate::gizmos_service::GizmosService;
use fruity_core::inject::Const;
use fruity_ecs::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;

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
            let position =
                if let Some(position) = entity_reader.get_component::<Position>("Position") {
                    position
                } else {
                    return;
                };

            let size = if let Some(size) = entity_reader.get_component::<Size>("Size") {
                size
            } else {
                return;
            };

            let bottom_left = position.pos;
            let top_right = position.pos + size.size;

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

                // Get the position origin
                let position_origin = {
                    let entity_reader = selected_entity.read();
                    let position = entity_reader.get_component::<Position>("Position").unwrap();
                    position.pos
                };

                drag_action.while_dragging(move |cursor_position, start_pos| {
                    let mut entity_writer = selected_entity.write();
                    let position = entity_writer
                        .get_component_mut::<Position>("Position")
                        .unwrap();

                    // Move the entity with the cursor
                    let cursor_movement = cursor_position - start_pos;
                    if move_x {
                        position.pos.x = position_origin.x + cursor_movement.x;
                    }

                    if move_y {
                        position.pos.y = position_origin.y + cursor_movement.y;
                    }
                });
            },
            move |fixed_x, fixed_y, drag_action| {
                let selected_entity = selected_entity_2.clone();

                // Get the position and the size origin
                let (position_origin, size_origin) = {
                    let entity_reader = selected_entity.read();
                    let position = entity_reader.get_component::<Position>("Position").unwrap();
                    let size = entity_reader.get_component::<Size>("Size").unwrap();

                    (position.pos, size.size)
                };

                drag_action.while_dragging(move |cursor_position, start_pos| {
                    let mut entity_writer = selected_entity.write();

                    let cursor_movement = cursor_position - start_pos;

                    // Move the entity with the cursor
                    let position = entity_writer
                        .get_component_mut::<Position>("Position")
                        .unwrap();
                    position.pos.x = if fixed_x {
                        position_origin.x
                    } else {
                        position_origin.x + cursor_movement.x
                    };

                    position.pos.y = if fixed_y {
                        position_origin.y
                    } else {
                        position_origin.y + cursor_movement.y
                    };

                    // Resize the entity with the cursor
                    let size = entity_writer.get_component_mut::<Size>("Size").unwrap();
                    size.size.x = if fixed_x {
                        size_origin.x + cursor_movement.x
                    } else {
                        size_origin.x - cursor_movement.x
                    };

                    size.size.y = if fixed_y {
                        size_origin.y + cursor_movement.y
                    } else {
                        size_origin.y - cursor_movement.y
                    };
                });
            },
        );
    }
}
