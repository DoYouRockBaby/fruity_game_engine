use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic::math::ALPHA;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::input_service::InputService;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

#[derive(Debug, FruityAny)]
pub struct GizmosService {
    input_service: ResourceReference<InputService>,
    graphic_2d_service: ResourceReference<Graphic2dService>,
}

impl GizmosService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> GizmosService {
        let input_service = resource_container.require::<InputService>();
        let graphic_2d_service = resource_container.require::<Graphic2dService>();

        GizmosService {
            input_service,
            graphic_2d_service,
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

        let is_hover = self.is_cursor_hover(&bottom_left, &top_right);
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_rect(bottom_left, top_right, 3, ALPHA, color, 1000);

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
        let graphic_2d_service = self.graphic_2d_service.read();
        let cursor_pos = graphic_2d_service.get_cursor_position();

        let is_hover = cursor_pos.in_triangle(&p1, &p2, &p3);
        let color = if is_hover { hover_color } else { color };

        graphic_2d_service.draw_line(p1, p2, 4, color, 1000);
        graphic_2d_service.draw_line(p2, p3, 4, color, 1000);
        graphic_2d_service.draw_line(p3, p1, 4, color, 1000);

        is_hover
    }

    pub fn draw_arrow_helper(
        &self,
        from: Vector2d,
        to: Vector2d,
        color: Color,
        hover_color: Color,
    ) -> bool {
        let graphic_2d_service = self.graphic_2d_service.read();
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
        graphic_2d_service.draw_line(from, to - normalise * 0.05, 4, color, 1000);

        is_hover
    }

    pub fn draw_move_helper<FMove>(
        &self,
        center: Vector2d,
        size: Vector2d,
        color: Color,
        hover_color: Color,
        on_move: FMove,
    ) where
        FMove: Fn(bool, bool, DragAction) + Send + Sync + 'static,
    {
        let move_handle_size = Vector2d::new(0.05, 0.05);
        let top_right = Vector2d::new(center.x + size.x / 2.0, center.y + size.y / 2.0);

        // Draw free move helper
        let is_hover_free_move =
            self.draw_square_helper(center, center + move_handle_size, color, hover_color);

        // Draw the X arrow
        let from = (center + Vector2d::new(top_right.x, center.y)) / 2.0;
        let to = Vector2d::new(top_right.x + 0.1, center.y);
        let is_hover_x_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Draw the Y arrow
        let from = (center + Vector2d::new(center.x, top_right.y)) / 2.0;
        let to = Vector2d::new(center.x, top_right.y + 0.1);
        let is_hover_y_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Implement the logic
        let input_service = self.input_service.read();
        let graphic_2d_service = self.graphic_2d_service.read();
        let cursor_pos = graphic_2d_service.get_cursor_position();

        // Handle moving
        let on_move = Arc::new(on_move);
        if is_hover_free_move && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, false, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(false, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        // Handle moving
        let on_move = Arc::new(on_move);
        if is_hover_free_move && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(true, false, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_move = on_move.clone();
            DragAction::start(
                move |action| on_move(false, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }
    }

    pub fn draw_resize_helper<FResize>(
        &self,
        corner1: Vector2d,
        corner2: Vector2d,
        color: Color,
        hover_color: Color,
        on_resize: FResize,
    ) where
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
        let resize_handle_size = Vector2d::new(0.025, 0.025);

        // Draw the boundings
        self.draw_square_helper(bottom_left, top_right, color, hover_color);

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

        // Implement the logic
        let input_service = self.input_service.read();
        let graphic_2d_service = self.graphic_2d_service.read();
        let cursor_pos = graphic_2d_service.get_cursor_position();

        // Handle resize
        let on_resize = Arc::new(on_resize);
        if is_hover_resize_top_right && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(true, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        let on_resize = Arc::new(on_resize);
        if is_hover_resize_top_left && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(false, true, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        let on_resize = Arc::new(on_resize);
        if is_hover_resize_bottom_right && input_service.is_source_pressed_this_frame("Mouse/Left")
        {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(true, false, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }

        if is_hover_resize_bottom_left && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let on_resize = on_resize.clone();
            DragAction::start(
                move |action| on_resize(false, false, action),
                cursor_pos,
                self.input_service.clone(),
                self.graphic_2d_service.clone(),
            );
        }
    }

    fn is_cursor_hover(&self, bottom_left: &Vector2d, top_right: &Vector2d) -> bool {
        let graphic_2d_service = self.graphic_2d_service.read();
        let cursor_pos = graphic_2d_service.get_cursor_position();

        // Check if the cursor is in the rect
        bottom_left.x <= cursor_pos.x
            && cursor_pos.x <= top_right.x
            && bottom_left.y <= cursor_pos.y
            && cursor_pos.y <= top_right.y
    }
}

pub struct DragAction {
    start_pos: Vector2d,
    input_service: ResourceReference<InputService>,
    graphic_2d_service: ResourceReference<Graphic2dService>,
}

impl DragAction {
    fn start<F>(
        callback: F,
        start_pos: Vector2d,
        input_service: ResourceReference<InputService>,
        graphic_2d_service: ResourceReference<Graphic2dService>,
    ) where
        F: Fn(DragAction) + Send + Sync + 'static,
    {
        // Create the darg action structure
        let drag_action = DragAction {
            start_pos,
            input_service,
            graphic_2d_service,
        };

        // Start the action in a specific thread
        callback(drag_action);
    }

    pub fn start_pos(&self) -> Vector2d {
        self.start_pos
    }

    pub fn while_dragging(self, callback: impl Fn(Vector2d, Vector2d) + Send + Sync + 'static) {
        spawn(move || {
            while self.is_dragging_button_pressed() {
                sleep(Duration::from_millis(20));
                callback(self.get_cursor_position(), self.start_pos());
            }
        });
    }

    fn is_dragging_button_pressed(&self) -> bool {
        let input_service = self.input_service.read();
        input_service.is_source_pressed("Mouse/Left")
    }

    pub fn get_cursor_position(&self) -> Vector2d {
        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.get_cursor_position()
    }
}

impl IntrospectObject for GizmosService {
    fn get_class_name(&self) -> String {
        "GizmosService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for GizmosService {}
