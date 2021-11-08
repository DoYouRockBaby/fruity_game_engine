use crate::gizmos_service::GizmosService;
use fruity_core::component::component_rwlock::ComponentRwLock;
use fruity_core::entity::entity::EntityId;
use fruity_core::entity::entity_manager::EntityManager;
use fruity_core::entity_type;
use fruity_core::service::service_guard::ServiceReadGuard;
use fruity_core::service::service_manager::ServiceManager;
use fruity_editor::hooks::use_global;
use fruity_editor::state::entity::EntityState;
use fruity_graphic::math::GREEN;
use fruity_graphic::math::RED;
use fruity_graphic_2d::components::position::Position;
use fruity_graphic_2d::components::size::Size;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::time::Duration;

pub fn draw_gizmos_2d(
    entity_id: &EntityId,
    position: ComponentRwLock,
    size: ComponentRwLock,
    gizmos_service: ServiceReadGuard<GizmosService>,
) {
    let entity = use_global::<EntityState>();

    /*let handle_color = theme_state.theme.surface_color();
    let handle_color = Color([
        handle_color.r,
        handle_color.g,
        handle_color.b,
        handle_color.a,
    ]);

    let handle_hover_color = theme_state.theme.hovered_color();
    let handle_hover_color = Color([
        handle_hover_color.r,
        handle_hover_color.g,
        handle_hover_color.b,
        handle_hover_color.a,
    ]);*/

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

                    while drag_action.is_dragging() {
                        sleep(Duration::from_millis(20));
                        let cursor_movement =
                            drag_action.get_cursor_position() - drag_action.start_pos();

                        let mut position = position.write();
                        let position = position.as_any_mut().downcast_mut::<Position>().unwrap();

                        if move_x {
                            position.pos.x = position_origin.x + cursor_movement.x;
                        }

                        if move_y {
                            position.pos.y = position_origin.y + cursor_movement.y;
                        }
                    }
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

                    while drag_action.is_dragging() {
                        sleep(Duration::from_millis(100));
                        let cursor_movement =
                            drag_action.get_cursor_position() - drag_action.start_pos();

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
                    }
                },
            );
        }
    }
}

pub fn draw_gizmos_2d_untyped(service_manager: Arc<RwLock<ServiceManager>>) {
    let service_manager = service_manager.read().unwrap();
    let entity_manager = service_manager.read::<EntityManager>();

    entity_manager
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

            let service1 = service_manager.read::<GizmosService>();
            draw_gizmos_2d(&components.entity_id(), position, size, service1);
        });
}