use crate::state::entity::EntityMessage;
use crate::state::Message;
use crate::state::State;
use iced::Alignment;
use iced::Checkbox;
use iced::Row;
use iced::TextInput;
use iced_wgpu::Renderer;
use iced_winit::scrollable;
use iced_winit::text_input;
use iced_winit::Element;
use iced_winit::Length;
use iced_winit::Scrollable;

pub struct EntityEditor {
    scroll: scrollable::State,
    name_input: text_input::State,
}

impl EntityEditor {
    pub fn new() -> EntityEditor {
        EntityEditor {
            scroll: scrollable::State::default(),
            name_input: text_input::State::default(),
        }
    }

    pub fn update(&mut self, _message: &Message) {}

    pub fn view(&mut self, state: &State) -> Element<Message, Renderer> {
        if let Some(entity) = &state.entity.selected_entity {
            let entity = entity.read();

            // Render entity head
            let enabled_checkbox = Checkbox::new(entity.enabled, "", |enabled| {
                Message::Entity(EntityMessage::SetEnabled(enabled))
            })
            .style(state.theme.theme);

            let name_input =
                TextInput::new(&mut self.name_input, "Name ...", &entity.name, |name| {
                    Message::Entity(EntityMessage::SetName(name))
                })
                .padding(10)
                .size(16)
                .width(Length::Fill)
                .style(state.theme.theme);

            let scrollable = Scrollable::new(&mut self.scroll)
                .padding(10)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Units(500))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .push(enabled_checkbox)
                        .push(name_input),
                )
                .style(state.theme.theme);

            // Render each components
            // TODO

            scrollable.into()
        } else {
            Row::new().into()
        }
    }
}
