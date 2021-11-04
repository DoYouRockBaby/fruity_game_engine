use crate::hooks::use_global;
use crate::hooks::use_state;
use crate::state::theme::ThemeState;
use crate::ui_element::iced::draw_element;
use crate::ui_element::pane::Pane;
use crate::ui_element::pane::PaneGrid;
use crate::ui_element::pane::UIPaneSide;
use crate::ui_element::Message;
use comp_state::CloneState;
use iced::Container as IcedContainer;
use iced::Length as IcedLength;
use iced::PaneGrid as IcedPaneGrid;
use iced::Row as IcedRow;
use iced::Text as IcedText;
use iced_wgpu::Renderer;
use iced_winit::pane_grid;
use iced_winit::pane_grid::Content as IcedContent;
use iced_winit::Element;
use std::sync::Arc;
use std::sync::Mutex;

pub fn draw_pane_grid<'a>(elem: PaneGrid) -> Element<'a, Message, Renderer> {
    // Initialize the pane grid state
    let panes = elem.panes.clone();
    let left_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Left)
            .collect::<Vec<_>>()
    });

    let panes = elem.panes.clone();
    let right_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Right)
            .collect::<Vec<_>>()
    });

    let panes = elem.panes.clone();
    let bottom_panes = use_state(|| {
        panes
            .into_iter()
            .filter(|pane| pane.default_side == UIPaneSide::Bottom)
            .collect::<Vec<_>>()
    });

    // Generate iced pane state
    let pane_grid_state = use_state(|| {
        Arc::new(Mutex::new(pane_grid::State::with_configuration(
            pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.8,
                a: Box::new(pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.2,
                    a: Box::new(pane_grid::Configuration::Pane(Some(UIPaneSide::Left))),
                    b: Box::new(pane_grid::Configuration::Split {
                        axis: pane_grid::Axis::Vertical,
                        ratio: 0.7,
                        a: Box::new(pane_grid::Configuration::Pane(None)),
                        b: Box::new(pane_grid::Configuration::Pane(Some(UIPaneSide::Right))),
                    }),
                }),
                b: Box::new(pane_grid::Configuration::Pane(Some(UIPaneSide::Bottom))),
            },
        )))
    });

    let pane_grid_state_ref = pane_grid_state.get();
    let mut pane_grid_state_ref = pane_grid_state_ref.lock().unwrap();

    // TODO: Try to find a way to remove that
    // Create a custom use_state for mutable references
    let pane_grid_state_ref = unsafe {
        std::mem::transmute::<
            &mut iced::pane_grid::State<Option<UIPaneSide>>,
            &mut iced::pane_grid::State<Option<UIPaneSide>>,
        >(&mut pane_grid_state_ref)
    };

    IcedPaneGrid::new(pane_grid_state_ref, |_id, pane| {
        let content: pane_grid::Content<Message, Renderer> = match pane {
            Some(UIPaneSide::Left) => draw_pane(left_panes.get()),
            Some(UIPaneSide::Right) => draw_pane(right_panes.get()),
            Some(UIPaneSide::Bottom) => draw_pane(bottom_panes.get()),
            None => pane_grid::Content::new(IcedRow::new()).into(),
        };

        content
    })
    .width(IcedLength::Fill)
    .height(IcedLength::Fill)
    /*.on_drag(move |event| {
        Message::Callback(Arc::new(move || {
            if let pane_grid::DragEvent::Dropped { pane, target } = event {
                let pane_grid_state_ref = pane_grid_state.get();
                let mut pane_grid_state_ref = pane_grid_state_ref.lock().unwrap();
                let pane1 = pane_grid_state_ref.get(&pane).unwrap();
                let pane2 = pane_grid_state_ref.get(&target).unwrap();
                // Avoid if one is a none pane
                if None != *pane1 && None != *pane2 {
                    pane_grid_state_ref.swap(&pane, &target);
                } else {
                }
            }
        }))
    })*/
    .on_resize(10, move |event| {
        Message::Callback(Arc::new(move || {
            let pane_grid_state_ref = pane_grid_state.get();
            let mut pane_grid_state_ref = pane_grid_state_ref.lock().unwrap();

            pane_grid_state_ref.resize(&event.split, event.ratio)
        }))
    })
    .into()
}

pub fn draw_pane<'a>(panes: Vec<Pane>) -> IcedContent<'a, Message, Renderer> {
    // TODO: Add tab system
    let first_pane = if let Some(pane) = panes.get(0) {
        pane
    } else {
        return pane_grid::Content::new(IcedRow::new()).into();
    };

    let theme_state = use_global::<ThemeState>();

    let title =
        IcedRow::with_children(vec![IcedText::new(&first_pane.title).size(16).into()]).spacing(5);

    let title_bar = pane_grid::TitleBar::new(title)
        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
        .padding(10)
        .style(theme_state.theme);

    let content = IcedContainer::new(
        IcedRow::with_children(vec![draw_element((first_pane.render)())]).padding(10),
    );

    pane_grid::Content::new(content)
        .title_bar(title_bar)
        .style(theme_state.theme.panel())
        .into()
}
