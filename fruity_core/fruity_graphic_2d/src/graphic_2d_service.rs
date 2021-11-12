use crate::Vector2d;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::math::Color;
use fruity_graphic::math::Matrix4;
use fruity_graphic::resources::material_resource::MaterialResource;

pub trait Graphic2dService: Resource {
    fn start_pass(&self, view_proj: Matrix4);
    fn end_pass(&self);
    fn draw_square(
        &self,
        pos: Vector2d,
        size: Vector2d,
        z_index: usize,
        material: ResourceReference<dyn MaterialResource>,
    );
    fn draw_line(&self, pos1: Vector2d, pos2: Vector2d, width: u32, color: Color, z_index: usize);
    /// Get the cursor position in the 2D world, take in care the camera transform
    fn get_cursor_position(&self) -> Vector2d;
}
