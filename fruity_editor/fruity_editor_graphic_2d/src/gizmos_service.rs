use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_input::drag_service::DragCallback;
use fruity_input::drag_service::DragEndCallback;
use fruity_input::drag_service::DragService;
use fruity_input::input_service::InputService;
use std::fmt::Debug;

#[derive(Debug, FruityAny)]
pub struct GizmosService {
    input_service: ResourceReference<InputService>,
    drag_service: ResourceReference<DragService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    graphic_2d_service: ResourceReference<Graphic2dService>,
}

impl GizmosService {
    pub fn new(resource_container: ResourceContainer) -> GizmosService {
        let input_service = resource_container.require::<InputService>();
        let drag_service = resource_container.require::<DragService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let graphic_2d_service = resource_container.require::<Graphic2dService>();

        GizmosService {
            input_service,
            drag_service,
            graphic_service,
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
        graphic_2d_service.draw_rect(bottom_left, top_right, 3, Color::alpha(), color, 1000);

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
        let cursor_pos = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_cursor_position()
        };

        let is_hover = cursor_pos.in_triangle(&p1, &p2, &p3);
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_line(p1, p2, 3, color, 1000);
        graphic_2d_service.draw_line(p2, p3, 3, color, 1000);
        graphic_2d_service.draw_line(p3, p1, 3, color, 1000);

        is_hover
    }

    pub fn draw_circle_helper(
        &self,
        center: Vector2d,
        radius: f32,
        color: Color,
        hover_color: Color,
    ) -> bool {
        let cursor_pos = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_cursor_position()
        };

        let is_hover = cursor_pos.in_circle(&center, &radius);
        let color = if is_hover { hover_color } else { color };

        let graphic_2d_service = self.graphic_2d_service.read();
        graphic_2d_service.draw_circle(center, radius, 3, color, Color::alpha(), 1000);

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

        // Get camera transform
        let camera_transform = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_camera_transform()
        };
        let camera_invert = camera_transform.invert();

        let is_hover = self.draw_triangle_helper(
            to - camera_invert * (normal * 0.025) - camera_invert * (normalise * 0.05),
            to + camera_invert * (normal * 0.025) - camera_invert * (normalise * 0.05),
            to,
            color,
            hover_color,
        );

        let color = if is_hover { hover_color } else { color };
        graphic_2d_service.draw_line(
            from,
            to - camera_invert * (normalise * 0.05),
            3,
            color,
            1000,
        );

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
        FMove: Fn(bool, bool) -> (DragCallback, DragEndCallback),
    {
        // Get camera transform
        let camera_transform = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_camera_transform()
        };
        let camera_invert = camera_transform.invert();

        let move_handle_size = camera_invert * Vector2d::new(0.05, 0.05);
        let move_handle_size = Vector2d::new(
            f32::min(move_handle_size.x, move_handle_size.y),
            f32::min(move_handle_size.x, move_handle_size.y),
        );

        let top_right = Vector2d::new(center.x + size.x / 2.0, center.y + size.y / 2.0);

        // Draw free move helper
        let is_hover_free_move =
            self.draw_square_helper(center, center + move_handle_size, color, hover_color);

        // Draw the X arrow
        let from = (center + Vector2d::new(top_right.x, center.y)) / 2.0;
        let to =
            Vector2d::new(top_right.x, center.y) + Vector2d::new(move_handle_size.x * 2.0, 0.0);
        let is_hover_x_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Draw the Y arrow
        let from = (center + Vector2d::new(center.x, top_right.y)) / 2.0;
        let to =
            Vector2d::new(center.x, top_right.y) + Vector2d::new(0.0, move_handle_size.y * 2.0);
        let is_hover_y_arrow = self.draw_arrow_helper(from, to, color, hover_color);

        // Implement the logic
        let input_service = self.input_service.read();

        // Handle moving
        if is_hover_free_move && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, true));
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, false));
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(false, true));
        }

        // Handle moving
        if is_hover_free_move && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, true));
        }

        if is_hover_x_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(true, false));
        }

        if is_hover_y_arrow && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_move(false, true));
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
        FResize: Fn(bool, bool) -> (DragCallback, DragEndCallback),
    {
        // Get camera transform
        let camera_transform = {
            let graphic_service = self.graphic_service.read();
            graphic_service.get_camera_transform()
        };
        let camera_invert = camera_transform.invert();

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
        let resize_handle_size = camera_invert * Vector2d::new(0.025, 0.025);
        let resize_handle_size = Vector2d::new(
            f32::min(resize_handle_size.x, resize_handle_size.y),
            f32::min(resize_handle_size.x, resize_handle_size.y),
        );

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

        // Handle resize
        if is_hover_resize_top_right && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(true, true));
        }

        if is_hover_resize_top_left && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(false, true));
        }

        if is_hover_resize_bottom_right && input_service.is_source_pressed_this_frame("Mouse/Left")
        {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(true, false));
        }

        if is_hover_resize_bottom_left && input_service.is_source_pressed_this_frame("Mouse/Left") {
            let drag_service_reader = self.drag_service.read();
            drag_service_reader.start_drag(|| on_resize(false, false));
        }
    }

    fn is_cursor_hover(&self, bottom_left: &Vector2d, top_right: &Vector2d) -> bool {
        let graphic_service = self.graphic_service.read();
        let cursor_pos = graphic_service.get_cursor_position();

        // Check if the cursor is in the rect
        bottom_left.x <= cursor_pos.x
            && cursor_pos.x <= top_right.x
            && bottom_left.y <= cursor_pos.y
            && cursor_pos.y <= top_right.y
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
