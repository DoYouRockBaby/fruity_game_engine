use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::graphics_manager::GraphicsManager;
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
    graphics_manager: ServiceRwLock<GraphicsManager>,
    graphics_2d_manager: ServiceRwLock<Graphics2dManager>,
}

impl GizmosService {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> GizmosService {
        let service_manager = service_manager.read().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
        let graphics_2d_manager = service_manager.get::<Graphics2dManager>().unwrap();

        GizmosService {
            graphics_manager,
            graphics_2d_manager,
        }
    }

    pub fn draw_square_helper(
        &self,
        corner1: Vector2d,
        corner2: Vector2d,
        color: Color,
        hover_color: Color,
    ) -> bool {
        let bottom_left = Vector2d::new(
            f32::min(corner1.x, corner2.x),
            f32::min(corner1.y, corner2.y),
        );
        let top_right = Vector2d::new(
            f32::max(corner1.x, corner2.x),
            f32::max(corner1.y, corner2.y),
        );

        let bottom_right = Vector2d::new(top_right.x, bottom_left.y);
        let top_left = Vector2d::new(bottom_left.x, top_right.y);

        let is_hover = self.is_cursor_hover(&bottom_left, &top_right);
        let color = if is_hover { color } else { hover_color };

        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        graphics_2d_manager.draw_line(bottom_left, bottom_right, 3, &color);
        graphics_2d_manager.draw_line(bottom_right, top_right, 3, &color);
        graphics_2d_manager.draw_line(top_right, top_left, 3, &color);
        graphics_2d_manager.draw_line(top_left, bottom_left, 3, &color);

        is_hover
    }

    pub fn draw_triangle_helper(
        &self,
        p1: Vector2d,
        p2: Vector2d,
        p3: Vector2d,
        color: Color,
        hover_color: Color,
    ) -> bool {
        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        let cursor_pos = graphics_2d_manager.get_cursor_position();

        let is_hover = cursor_pos.in_triangle(&p1, &p2, &p3);
        let color = if is_hover { color } else { hover_color };

        graphics_2d_manager.draw_line(p1, p2, 3, &color);
        graphics_2d_manager.draw_line(p2, p3, 3, &color);
        graphics_2d_manager.draw_line(p3, p1, 3, &color);

        is_hover
    }

    pub fn draw_arrow_helper(
        &self,
        from: Vector2d,
        to: Vector2d,
        color: Color,
        hover_color: Color,
    ) -> bool {
        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        let normalise = (to - from).normalise();
        let normal = normalise.normal();

        let is_hover = self.draw_triangle_helper(
            to - normal * 0.025 - normalise * 0.05,
            to + normal * 0.025 - normalise * 0.05,
            to,
            color,
            hover_color,
        );

        let color = if is_hover { color } else { hover_color };
        graphics_2d_manager.draw_line(from, to - normalise * 0.05, 3, &color);

        is_hover
    }

    pub fn draw_resize_helper(
        &self,
        corner1: Vector2d,
        corner2: Vector2d,
        color: Color,
        hover_color: Color,
    ) {
        let bottom_left = Vector2d::new(
            f32::min(corner1.x, corner2.x),
            f32::min(corner1.y, corner2.y),
        );
        let top_right = Vector2d::new(
            f32::max(corner1.x, corner2.x),
            f32::max(corner1.y, corner2.y),
        );

        let bottom_right = Vector2d::new(top_right.x, bottom_left.y);
        let top_left = Vector2d::new(bottom_left.x, top_right.y);
        let center = (bottom_left + top_right) / 2.0;
        let resize_handle_size = Vector2d::new(0.025, 0.025);

        // Draw bottom left
        self.draw_square_helper(
            bottom_left,
            bottom_left + resize_handle_size,
            color,
            hover_color,
        );

        // Draw bottom right
        self.draw_square_helper(
            bottom_right,
            bottom_right + Vector2d::new(-resize_handle_size.x, resize_handle_size.y),
            color,
            hover_color,
        );

        // Draw top left
        self.draw_square_helper(
            top_left,
            top_left + Vector2d::new(resize_handle_size.x, -resize_handle_size.y),
            color,
            hover_color,
        );

        // Draw top right
        self.draw_square_helper(
            top_right,
            top_right - resize_handle_size,
            color,
            hover_color,
        );

        // Draw the X arrow
        let from = (center + Vector2d::new(top_right.x, center.y)) / 2.0;
        let to = Vector2d::new(bottom_right.x + 0.1, center.y);
        self.draw_arrow_helper(from, to, color, hover_color);

        // Draw the Y arrow
        let from = (center + Vector2d::new(center.x, top_right.y)) / 2.0;
        let to = Vector2d::new(center.x, top_left.y + 0.1);
        self.draw_arrow_helper(from, to, color, hover_color);
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
