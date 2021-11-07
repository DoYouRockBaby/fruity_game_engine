use crate::components::root::root_component;
use crate::hooks::declare_global;
use crate::state::entity::EntityState;
use crate::state::file_explorer::FileExplorerState;
use crate::state::theme::ThemeState;
use crate::state::world::WorldState;
use crate::ui_element::egui::draw_element;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::Platform;
use fruity_core::service::service_manager::ServiceManager;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Application {}

impl Application {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        declare_global(WorldState::new(service_manager));
        declare_global(ThemeState::default());
        declare_global(EntityState::default());
        declare_global(FileExplorerState::default());

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
            draw_element(root_component(), ui, ctx);
        });
    }
}
