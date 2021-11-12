use crate::gizmos_service::GizmosService;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity::EntityId;
use fruity_core::entity::entity_service::EntityService;
use fruity_core::entity_type;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::entity::EntityState;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;
use rayon::prelude::*;
use std::sync::Arc;

pub fn draw_gizmos_2d(
    entity_id: &EntityId,
    position: ComponentRwLock,
    size: ComponentRwLock,
    gizmos_service: ResourceReference<GizmosService>,
) {
    let entity = use_global::<EntityState>();
    let gizmos_service = gizmos_service.read();

    if let Some(selected_entity) = &entity.selected_entity {
        let selected_entity_id = {
            let selected_entity = selected_entity.read();
            selected_entity.entity_id
        };

        if selected_entity_id == *entity_id {
            let bottom_left = {
                let position = position.read();
                position
                    .as_any_ref()
                    .downcast_ref::<Position>()
                    .unwrap()
                    .pos
            };

            let top_right = {
                let size = size.read();
                bottom_left + size.as_any_ref().downcast_ref::<Size>().unwrap().size
            };

            let position_2 = position.clone();
            gizmos_service.draw_resize_helper(
                bottom_left,
                top_right,
                GREEN,
                RED,
                move |move_x, move_y, drag_action| {
                    let position = position.clone();
                    let position_origin = {
                        let position = position.read();
                        position
                            .as_any_ref()
                            .downcast_ref::<Position>()
                            .unwrap()
                            .pos
                    };

                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let cursor_movement = cursor_position - start_pos;

                        let mut position = position.write();
                        let position = position.as_any_mut().downcast_mut::<Position>().unwrap();

                        if move_x {
                            position.pos.x = position_origin.x + cursor_movement.x;
                        }

                        if move_y {
                            position.pos.y = position_origin.y + cursor_movement.y;
                        }
                    });
                },
                move |fixed_x, fixed_y, drag_action| {
                    let position = position_2.clone();
                    let position_origin = {
                        let position = position.read();
                        position
                            .as_any_ref()
                            .downcast_ref::<Position>()
                            .unwrap()
                            .pos
                    };

                    let size_origin = {
                        let size = size.read();
                        size.as_any_ref().downcast_ref::<Size>().unwrap().size
                    };

                    let size = size.clone();
                    drag_action.while_dragging(move |cursor_position, start_pos| {
                        let cursor_movement = cursor_position - start_pos;

                        let mut position_writer = position.write();
                        let position = position_writer
                            .as_any_mut()
                            .downcast_mut::<Position>()
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

                        std::mem::drop(position_writer);

                        let mut size = size.write();
                        let size = size.as_any_mut().downcast_mut::<Size>().unwrap();

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
}

pub fn draw_gizmos_2d_untyped(resource_container: Arc<ResourceContainer>) {
    let resource1 = resource_container.require::<GizmosService>("gizmos_service");

    let entity_service = resource_container.require::<EntityService>("entity_service");
    let entity_service = entity_service.read();

    entity_service
        .iter_components(entity_type!["Position", "Size"])
        .par_bridge()
        .for_each(|components| {
            let position = match components.get(0) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            let size = match components.get(1) {
                Some(component) => component,
                None => {
                    log::error!("Tried to launch a system with a component that was not provided");
                    return;
                }
            };

            draw_gizmos_2d(&components.entity_id(), position, size, resource1.clone());
        });
}
