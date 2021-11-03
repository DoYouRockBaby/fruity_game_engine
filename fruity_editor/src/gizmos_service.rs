use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphics_2d_manager::Graphics2dManager;
use fruity_graphic_2d::math::vector2d::Vector2d;
use fruity_input::input_manager::InputManager;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

#[derive(Debug, FruityAny)]
pub struct GizmosService {
    input_manager: ServiceRwLock<InputManager>,
    graphics_manager: ServiceRwLock<GraphicsManager>,
    graphics_2d_manager: ServiceRwLock<Graphics2dManager>,
}

impl GizmosService {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> GizmosService {
        let service_manager = service_manager.read().unwrap();
        let input_manager = service_manager.get::<InputManager>().unwrap();
        let graphics_manager = service_manager.get::<GraphicsManager>().unwrap();
        let graphics_2d_manager = service_manager.get::<Graphics2dManager>().unwrap();

        GizmosService {
            input_manager,
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
        let color = if is_hover { hover_color } else { color };

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
        let color = if is_hover { hover_color } else { color };

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

        let color = if is_hover { hover_color } else { color };
        graphics_2d_manager.draw_line(from, to - normalise * 0.05, 3, &color);

        is_hover
    }

    pub fn draw_resize_helper<FMove, FResize>(
        &self,
        corner1: Vector2d,
        corner2: Vector2d,
        color: Color,
        hover_color: Color,
        on_move: FMove,
        on_resize: FResize,
    ) where
        FMove: Fn(bool, bool, DragAction) + Send + Sync + 'static,
        FResize: Fn(bool, bool, DragAction) + Send + Sync + 'static,
    {
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

        // Draw the boundings
        let is_hover_bounds = self.draw_square_helper(bottom_left, top_right, color, hover_color);

        // Draw bottom left
        let is_hover_resize_bottom_left = self.draw_square_helper(
            bottom_left,
            bottom_left + resize_handle_size,
            color,
            hover_color,
        );

        // Draw bottom right
        let is_hover_resize_bottom_right = self.draw_square_helper(
            bottom_right,
            bottom_right + Vector2d::new(-resize_handle_size.x, resize_handle_size.y),
            color,
            hover_color,
        );

        // Draw top left
        let is_hover_resize_top_left = self.draw_square_helper(
            top_left,
            top_left + Vector2d::new(resize_handle_size.x, -resize_handle_size.y),
            color,
            hover_color,
        );

        // Draw top right
        let is_hover_resize_top_right = self.draw_square_helper(
            top_right,
            top_right - resize_handle_size,
            color,
            hover_color,
        );

        // Draw the X arrow
        let from = (center + Vector2d::new(top_right.x, center.y)) / 2.0;
        let to = Vector2d::new(bottom_right.x + 0.1, center.y);
        let is_hover_x_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Draw the Y arrow
        let from = (center + Vector2d::new(center.x, top_right.y)) / 2.0;
        let to = Vector2d::new(center.x, top_left.y + 0.1);
        let is_hover_y_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Implement the logic
        let input_manager = self.input_manager.read().unwrap();
        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        let cursor_pos = graphics_2d_manager.get_cursor_position();

        // Handle moving
        let on_move = Arc::new(on_move);
        if is_hover_bounds
            && !is_hover_resize_top_right
            && !is_hover_resize_bottom_right
            && !is_hover_resize_top_left
            && !is_hover_resize_bottom_left
            && input_manager.is_source_pressed_this_frame("Mouse/Left")
        {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, true, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        if is_hover_x_arrow && input_manager.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, false, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        if is_hover_y_arrow && input_manager.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(false, true, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        // Handle resize
        let on_resize = Arc::new(on_resize);
        if is_hover_resize_top_right && input_manager.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(true, true, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        let on_resize = Arc::new(on_resize);
        if is_hover_resize_top_left && input_manager.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(false, true, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        let on_resize = Arc::new(on_resize);
        if is_hover_resize_bottom_right && input_manager.is_source_pressed_this_frame("Mouse/Left")
        {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(true, false, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }

        if is_hover_resize_bottom_left && input_manager.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(false, false, action),
                cursor_pos,
                self.input_manager.clone(),
                self.graphics_2d_manager.clone(),
            );
        }
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

pub struct DragAction {
    start_pos: Vector2d,
    is_dragging: Arc<RwLock<bool>>,
    graphics_2d_manager: ServiceRwLock<Graphics2dManager>,
}

impl DragAction {
    fn start<F>(
        callback: F,
        start_pos: Vector2d,
        input_manager: ServiceRwLock<InputManager>,
        graphics_2d_manager: ServiceRwLock<Graphics2dManager>,
    ) where
        F: Fn(DragAction) + Send + Sync + 'static,
    {
        let is_dragging = Arc::new(RwLock::new(true));
        let is_dragging_2 = is_dragging.clone();

        // This thread will observe if were still dragging
        spawn(move || {
            let is_dragging = is_dragging.clone();
            let mut is_mouse_pressed = true;
            while is_mouse_pressed {
                sleep(Duration::from_millis(100));
                let input_manager = input_manager.read().unwrap();
                is_mouse_pressed = input_manager.is_source_pressed("Mouse/Left");
            }

            let mut is_dragging = is_dragging.write().unwrap();
            *is_dragging = false;
        });

        // Create the darg action structure
        let drag_action = DragAction {
            start_pos,
            is_dragging: is_dragging_2,
            graphics_2d_manager,
        };

        // Start the action in a specific thread
        spawn(move || callback(drag_action));
    }

    pub fn start_pos(&self) -> Vector2d {
        self.start_pos
    }

    pub fn is_dragging(&self) -> bool {
        let is_dragging = self.is_dragging.read().unwrap();
        *is_dragging
    }

    pub fn get_cursor_position(&self) -> Vector2d {
        let graphics_2d_manager = self.graphics_2d_manager.read().unwrap();
        graphics_2d_manager.get_cursor_position()
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
