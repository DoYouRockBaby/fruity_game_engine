use crate::ui_element::draw_element;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::Platform;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_editor::components::root::root_component;
use std::sync::Arc;

pub struct Application {}

impl Application {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        Application {}
    }
}

pub struct DrawContext<'s> {
    pub device: &'s wgpu::Device,
    pub platform: &'s Platform,
    pub egui_rpass: &'s mut RenderPass,
}

impl Application {
    pub fn draw(&mut self, ctx: &mut DrawContext) {
        egui::Area::new("root").show(&ctx.platform.context(), |ui| {
            root_component()
                .into_iter()
                .for_each(|child| draw_element(child, ui, ctx));
        });
    }
}
