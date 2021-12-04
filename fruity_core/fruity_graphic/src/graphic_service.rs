use crate::math::material_reference::MaterialReference;
use crate::math::matrix4::Matrix4;
use crate::resources::material_resource::MaterialResource;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::signal::Signal;

pub trait GraphicService: Resource {
    fn start_draw(&mut self);
    fn end_draw(&mut self);
    fn start_pass(&self);
    fn end_pass(&self);
    fn update_camera(&mut self, view_proj: Matrix4);
    fn get_camera_transform(&self) -> &Matrix4;
    fn resize(&mut self, width: usize, height: usize);
    fn on_before_draw_end(&self) -> &Signal<()>;
    fn on_after_draw_end(&self) -> &Signal<()>;
    fn material_reference_from_resource_reference(
        &self,
        resource_reference: ResourceReference<MaterialResource>,
    ) -> Box<dyn MaterialReference>;
}
