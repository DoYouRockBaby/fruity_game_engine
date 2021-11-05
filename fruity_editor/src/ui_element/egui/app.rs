use crate::components::root::root_component;
use crate::hooks::declare_global;
use crate::state::entity::EntityState;
use crate::state::theme::ThemeState;
use crate::state::world::WorldState;
use crate::ui_element::egui::draw_element;
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

        Application {}
    }
}

impl Application {
    pub fn draw(&mut self, platform: &Platform) {
        egui::Area::new("root").show(&platform.context(), |ui| {
            draw_element(root_component(), ui);
        });
    }
}
