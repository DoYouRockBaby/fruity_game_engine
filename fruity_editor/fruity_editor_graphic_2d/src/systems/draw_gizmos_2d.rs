use crate::gizmos_service::GizmosService;
use fruity_core::inject::Const;
use fruity_editor::hooks::use_global;
use fruity_editor::state::entity::EntityState;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;

pub fn draw_gizmos_2d(gizmos_service: Const<GizmosService>) {
    let entity = use_global::<EntityState>();

    if let Some(selected_entity) = &entity.selected_entity {
        let (bottom_left, top_right) = {
            let entity_reader = selected_entity.read();
            let position = entity_reader.get_component::<Position>("Position").unwrap();
            let size = entity_reader.get_component::<Size>("Size").unwrap();

            let bottom_left = position.pos;
            let top_right = position.pos + size.size;

            (bottom_left, top_right)
        };

        gizmos_service.draw_resize_helper(
            bottom_left,
            top_right,
            GREEN,
            RED,
            move |move_x, move_y, drag_action| {
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
                let (position_origin, size_origin) = {
                    let entity_reader = selected_entity.read();
                    let position = entity_reader.get_component::<Position>("Position").unwrap();
                    let size = entity_reader.get_component::<Size>("Size").unwrap();

                    (position.pos, size.size)
                };

                drag_action.while_dragging(move |cursor_position, start_pos| {
                    let mut entity_writer = selected_entity.write();

                    let cursor_movement = cursor_position - start_pos;

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
