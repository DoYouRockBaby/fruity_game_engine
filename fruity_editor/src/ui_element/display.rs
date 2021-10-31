use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use iced::Alignment as IcedAlignment;
use iced::Length as IcedLength;

impl Into<IcedAlignment> for UIAlign {
    fn into(self) -> IcedAlignment {
        match self {
            UIAlign::Start => IcedAlignment::Start,
            UIAlign::Center => IcedAlignment::Center,
            UIAlign::End => IcedAlignment::End,
        }
    }
}

impl Into<IcedLength> for UISize {
    fn into(self) -> IcedLength {
        match self {
            UISize::Fill => IcedLength::Fill,
            UISize::FillPortion(portion) => IcedLength::FillPortion(portion),
            UISize::Shrink => IcedLength::Shrink,
            UISize::Units(unit) => IcedLength::Units(unit),
        }
    }
}

pub struct Text {
    pub text: String,
    pub width: UISize,
    pub height: UISize,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "".to_string(),
            width: UISize::Fill,
            height: UISize::Shrink,
        }
    }
}

impl UIWidget for Text {
    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
