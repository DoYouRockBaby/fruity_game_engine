use fruity_core::resource::resource::Resource;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;

pub trait Graphic2dService: Resource {
    fn start_pass(&self, view_proj: Matrix4);
    fn end_pass(&self);
    fn draw_square(&self, transform: Matrix3, z_index: usize, material: &dyn MaterialReference);
    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: Color, z_index: usize);
    /// Get the cursor position in the 2D world, take in care the camera transform
    fn get_cursor_position(&self) -> Vector2d;
}
