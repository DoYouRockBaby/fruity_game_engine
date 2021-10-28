use crate::state::theme::ThemeMessage;
use crate::state::update_state;
use crate::state::Message;
use crate::state::State;
use iced::{
    executor, Alignment, Application, Column, Command, Container, Element,
    Length, Radio, Row, Rule, Settings, Text,
};

mod state;
mod style;

pub fn main() -> iced::Result {
    FruityEditor::run(Settings::default())
}

#[derive(Default)]
struct FruityEditor {
    state: State,
}

impl Application for FruityEditor {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (FruityEditor, Command<Self::Message>) {
        (FruityEditor::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("FruityEditor - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        update_state(&mut self.state, message);

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let choose_theme = style::Theme::ALL.iter().fold(
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

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(600)
            .push(choose_theme)
            .push(Rule::horizontal(38).style(self.state.theme.theme))
            .push(
                Row::new()
                    .spacing(10)
                    .height(Length::Units(100))
                    .align_items(Alignment::Center)
                    .push(Rule::vertical(38).style(self.state.theme.theme)),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.state.theme.theme)
            .into()
    }
}
