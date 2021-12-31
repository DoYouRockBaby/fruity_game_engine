use crate::math::material_reference::MaterialReference;
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
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::signal::Signal;

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
    fn resize(&mut self, width: usize, height: usize);
    fn draw_mesh(
        &self,
        identifier: u64,
        mesh: ResourceReference<dyn MeshResource>,
        material: &dyn MaterialReference,
        z_index: usize,
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
    fn create_material_reference(
        &self,
        resource_reference: ResourceReference<dyn MaterialResource>,
    ) -> Box<dyn MaterialReference>;
    fn on_before_draw_end(&self) -> &Signal<()>;
    fn on_after_draw_end(&self) -> &Signal<()>;
}
