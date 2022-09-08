use crate::ui_element::draw_element;
use crate::SecondaryActionState;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::Platform;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_editor::components::root::root_component;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::hooks::use_read_service;

pub struct Application {}

impl Application {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Application {}
    }
}

pub struct DrawContext<'s> {
    pub device: &'s wgpu::Device,
    pub platform: &'s Platform,
    pub egui_rpass: &'s mut RenderPass,
}

impl Application {
    pub fn draw(&mut self, ctx: &UIContext, draw_ctx: &mut DrawContext) {
        let mut local_ctx = ctx.clone();

        egui::Area::new("root").show(&draw_ctx.platform.context(), |ui| {
            root_component(&mut local_ctx)
                .into_iter()
                .for_each(|child| draw_element(child, &mut local_ctx, ui, draw_ctx));

            // Display the secondary click menu
            let secondary_action_state = use_read_service::<SecondaryActionState>(&ctx);
            secondary_action_state.draw_secondary_actions(&local_ctx, ui);
        });
    }
}
