use crate::math::matrix4::Matrix4;
use crate::math::Color;
use crate::resources::material_resource::MaterialResource;
use crate::resources::material_resource::MaterialResourceSettings;
use crate::resources::mesh_resource::MeshResource;
use crate::resources::mesh_resource::MeshResourceSettings;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::shader_resource::ShaderResourceSettings;
use crate::resources::texture_resource::TextureResource;
use crate::resources::texture_resource::TextureResourceSettings;
use crate::Vector2d;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::signal::Signal;
use std::collections::HashMap;

pub enum MaterialParam {
    UInt(u32),
    Int(i32),
    Float(f32),
    Vector2(Vector2d),
    Color(Color),
    Rect {
        bottom_left: Vector2d,
        top_right: Vector2d,
    },
    Matrix4(Matrix4),
}

pub trait GraphicService: Resource {
    fn start_draw(&mut self);
    fn end_draw(&mut self);
    fn render_scene(
        &self,
        view_proj: Matrix4,
        background_color: Color,
        target: Option<ResourceReference<dyn TextureResource>>,
    );
    fn get_camera_transform(&self) -> Matrix4;
    fn resize(&mut self, width: u32, height: u32);
    fn draw_mesh(
        &self,
        identifier: u64,
        mesh: ResourceReference<dyn MeshResource>,
        material: ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    );
    fn create_mesh_resource(
        &self,
        identifier: &str,
        params: MeshResourceSettings,
    ) -> Result<Box<dyn MeshResource>, String>;
    fn create_shader_resource(
        &self,
        identifier: &str,
        contents: String,
        params: ShaderResourceSettings,
    ) -> Result<Box<dyn ShaderResource>, String>;
    fn create_texture_resource(
        &self,
        identifier: &str,
        contents: &[u8],
        params: TextureResourceSettings,
    ) -> Result<Box<dyn TextureResource>, String>;
    fn create_material_resource(
        &self,
        identifier: &str,
        params: MaterialResourceSettings,
    ) -> Result<Box<dyn MaterialResource>, String>;
    fn on_before_draw_end(&self) -> &Signal<()>;
    fn on_after_draw_end(&self) -> &Signal<()>;
    fn get_viewport_position(&self, x: u32, y: u32) -> Vector2d;
    fn get_cursor_position(&self) -> Vector2d;
    fn is_cursor_hover_scene(&self) -> bool;
    fn get_viewport_offset(&self) -> (u32, u32);
    fn set_viewport_offset(&self, x: u32, y: u32);
    fn get_viewport_size(&self) -> (u32, u32);
    fn set_viewport_size(&self, x: u32, y: u32);
}
