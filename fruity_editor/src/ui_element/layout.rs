use crate::hooks::use_global;
use crate::state::theme::ThemeState;
use crate::ui_element::Message;
use crate::ui_element::UIAlign;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use iced::Column as IcedColumn;
use iced::Container as IcedContainer;
use iced::Row as IcedRow;
use iced_wgpu::Renderer;
use iced_winit::Element;

pub struct Empty {}

impl UIWidget for Empty {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        IcedRow::new().into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Row {
    pub children: Vec<UIElement>,
    pub align: UIAlign,
}

impl UIWidget for Row {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let theme_state = use_global::<ThemeState>();

        IcedContainer::new(self.children.iter().fold(
            IcedRow::new().align_items(self.align.clone().into()),
            |row, element| row.push(element.draw()),
        ))
        .style(theme_state.theme)
        .into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}

pub struct Column {
    pub children: Vec<UIElement>,
    pub align: UIAlign,
}

impl UIWidget for Column {
    fn draw<'a>(&self) -> Element<'a, Message, Renderer> {
        let theme_state = use_global::<ThemeState>();

        IcedContainer::new(self.children.iter().fold(
            IcedColumn::new().align_items(self.align.clone().into()),
            |row, element| row.push(element.draw()),
        ))
        .style(theme_state.theme)
        .into()
    }

    fn elem(self) -> UIElement {
        UIElement {
            root: Box::new(self),
        }
    }
}
