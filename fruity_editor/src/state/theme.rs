use crate::style::Theme;

#[derive(Debug, Default)]
pub struct ThemeState {
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum ThemeMessage {
    ThemeChanged(Theme),
}

pub fn update_theme(state: &mut ThemeState, message: ThemeMessage) {
    match message {
        ThemeMessage::ThemeChanged(theme) => state.theme = theme,
    }
}