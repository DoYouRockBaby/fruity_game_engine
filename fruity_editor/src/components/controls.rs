use crate::components::entity::entity_list::EntityList;
use crate::state::theme::ThemeMessage;
use crate::state::update_state;
use crate::state::Message;
use crate::state::State;
use crate::style::Theme;
use crate::World;
use iced_wgpu::Renderer;
use iced_winit::Alignment;
use iced_winit::Color;
use iced_winit::Column;
use iced_winit::Command;
use iced_winit::Element;
use iced_winit::Length;
use iced_winit::Program;
use iced_winit::Radio;
use iced_winit::Row;
use iced_winit::Text;

pub struct Controls {
    state: State,
    entity_list: EntityList,
}

impl Controls {
    pub fn new(world: &World) -> Self {
        Controls {
            state: State::new(world),
            entity_list: EntityList::default(),
        }
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        update_state(&mut self.state, message.clone());
        self.entity_list.update(message.clone());

        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let choose_theme = Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose a theme:")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        &format!("{:?}", theme),
                        Some(self.state.theme.theme),
                        |theme| Message::Theme(ThemeMessage::ThemeChanged(theme)),
                    )
                    .style(self.state.theme.theme),
                )
            },
        );

        /*let content = Column::new()
            .width(Length::Fill)
            .align_items(Alignment::End)
            .push(choose_theme)
            .push(Rule::horizontal(38).style(self.state.theme.theme))
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(Rule::vertical(38).style(self.state.theme.theme)),
            )
            .push(self.entity_list.view(&self.state));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.state.theme.theme)
            .into()*/

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Start)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Start)
                    .push(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(Text::new("Background color").color(Color::WHITE))
                            .push(choose_theme)
                            .push(self.entity_list.view(&self.state)),
                    ),
            )
            .into()
    }
}
