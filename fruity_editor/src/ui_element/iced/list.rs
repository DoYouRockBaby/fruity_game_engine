use crate::hooks::topo;
use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::state::theme::ThemeState;
use crate::ui_element::iced::draw_element;
use crate::ui_element::list::ListView;
use crate::ui_element::Message;
use comp_state::CloneState;
use iced::button;
use iced::scrollable;
use iced::Button as IcedButton;
use iced::Container as IcedContainer;
use iced::Length as IcedLength;
use iced::Scrollable as IcedScrollable;
use iced_wgpu::Renderer;
use iced_winit::Element;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[topo::nested]
pub fn draw_list_view<'a>(elem: ListView) -> Element<'a, Message, Renderer> {
    let theme_state = use_global::<ThemeState>();
    let scroll_state = use_state(|| Rc::<RefCell<scrollable::State>>::default());
    let list_state = use_state(|| Rc::<RefCell<HashMap<usize, button::State>>>::default());

    let scroll_state = scroll_state.get();
    let mut scroll_state = scroll_state.borrow_mut();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let scroll_state = unsafe {
        std::mem::transmute::<&mut iced::scrollable::State, &mut iced::scrollable::State>(
            &mut scroll_state,
        )
    };

    let old_list_state = list_state.get();
    let mut old_list_state = old_list_state.borrow_mut();

    // Update list button states from keys
    let keys = elem
        .items
        .iter()
        .map(|item| (elem.get_key)(item.deref()))
        .collect::<Vec<_>>();

    let new_list_state = keys
        .into_iter()
        .fold(HashMap::new(), |mut new_list_state, key| {
            let item_state = match old_list_state.remove(&key) {
                Some(item_state) => item_state,
                None => button::State::default(),
            };
            new_list_state.insert(key, item_state);
            new_list_state
        });

    let new_list_state = Rc::new(RefCell::new(new_list_state));
    list_state.set(new_list_state);

    let list_state = list_state.get();
    let mut list_state = list_state.borrow_mut();

    let get_key = elem.get_key;
    let render_item = elem.render_item;
    let on_clicked = elem.on_clicked;
    let content = elem.items.into_iter().fold(
        IcedScrollable::new(scroll_state)
            .width(IcedLength::Fill)
            .height(IcedLength::Units(500))
            .style(theme_state.theme),
        |scrollable, item| {
            // TODO: Try to find a way to remove that
            // Create a custom use_state for mutable references
            let list_state = unsafe {
                std::mem::transmute::<
                    &mut HashMap<usize, button::State>,
                    &mut HashMap<usize, button::State>,
                >(&mut list_state)
            };

            scrollable.push({
                let key = get_key(item.deref());
                let rendered_item = render_item(item.deref());
                let item_state = list_state.get_mut(&key).unwrap();
                let item: Element<Message, Renderer> =
                    IcedButton::new(item_state, draw_element(rendered_item))
                        .style(theme_state.theme.list_item())
                        .on_press(Message::AnyChanged(on_clicked.clone(), item.clone()))
                        .width(IcedLength::Fill)
                        .into();
                item
            })
        },
    );

    IcedContainer::new(content)
        .style(theme_state.theme.list_view())
        .width(IcedLength::Fill)
        .height(IcedLength::Fill)
        .into()
}
