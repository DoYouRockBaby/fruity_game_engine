use crate::hooks::topo;
use crate::ui_element::iced::draw_element;
use crate::ui_element::input::Button;
use crate::ui_element::layout::Container;
use crate::ui_element::layout::Row;
use crate::ui_element::menu::Menu;
use crate::ui_element::Message;
use crate::ui_element::UISize;
use crate::ui_element::UIWidget;
use iced_wgpu::Renderer;
use iced_winit::Element;
use std::sync::Arc;

#[topo::nested]
pub fn draw_menu<'a>(elem: Menu) -> Element<'a, Message, Renderer> {
    let menu_row = Container {
        child: Row {
            children: elem
                .sections
                .into_iter()
                .map(|section| {
                    Button {
                        label: section.label,
                        on_click: Arc::new(|| ()),
                    }
                    .elem()
                })
                .collect::<Vec<_>>(),
            ..Default::default()
        }
        .elem(),
        height: UISize::Shrink,
        ..Default::default()
    }
    .elem();

    draw_element(menu_row)
}
