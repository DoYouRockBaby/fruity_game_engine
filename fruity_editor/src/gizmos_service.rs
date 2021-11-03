use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphics_2d_manager::Graphics2dManager;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, FruityAny)]
pub struct GizmosService {
    graphics_2d_manager: ServiceRwLock<Graphics2dManager>,
}

impl GizmosService {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> GizmosService {
        let service_manager = service_manager.read().unwrap();
        let graphics_2d_manager = service_manager.get::<Graphics2dManager>().unwrap();

        GizmosService {
            graphics_2d_manager,
        }
    }

    pub fn draw_square_helper(
        &self,
        bottom_left: Vector2d,
        top_right: Vector2d,
        color: Color,
        hover_color: Color,
    ) {
        let bottom_right = Vector2d::new(top_right.x, bottom_left.y);
        let top_left = Vector2d::new(bottom_left.x, top_right.y);

        let color = if self.is_cursor_hover(&bottom_left, &top_right) {
            color
        } else {
            hover_color
        };

        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        graphics_2d_manager.draw_line(bottom_left, bottom_right, 3, &color);
        graphics_2d_manager.draw_line(bottom_right, top_right, 3, &color);
        graphics_2d_manager.draw_line(top_right, top_left, 3, &color);
        graphics_2d_manager.draw_line(top_left, bottom_left, 3, &color);
    }

    fn is_cursor_hover(&self, bottom_left: &Vector2d, top_right: &Vector2d) -> bool {
        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        let cursor_pos = graphics_2d_manager.get_cursor_position();

        // Check if the cursor is in the rect
        bottom_left.x <= cursor_pos.x
            && cursor_pos.x <= top_right.x
            && bottom_left.y <= cursor_pos.y
            && cursor_pos.y <= top_right.y
    }
}

impl IntrospectObject for GizmosService {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for GizmosService {}
