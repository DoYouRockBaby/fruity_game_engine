use crate::components::root::root_component;
use crate::hooks::declare_global;
use crate::state::entity::EntityState;
use crate::state::handle_message;
use crate::state::theme::ThemeState;
use crate::state::world::WorldState;
use crate::ui_element::iced::draw_element;
use crate::ui_element::Message;
use fruity_core::service::service_manager::ServiceManager;
use iced::Command;
use iced_wgpu::Renderer;
use iced_winit::Element;
use iced_winit::Program as IcedProgram;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Program {}

impl Program {
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> Self {
        declare_global(WorldState::new(service_manager));
        declare_global(ThemeState::default());
        declare_global(EntityState::default());

        Program {}
    }
}

impl IcedProgram for Program {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        handle_message(message.clone());
        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        draw_element(root_component())
    }
}
