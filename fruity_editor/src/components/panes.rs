use crate::components::entity::entity_edit::entity_edit_component;
use crate::components::entity::entity_list::entity_list_component;
use crate::hooks::declare_global;
use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::state::handle_message;
use crate::state::theme::ThemeState;
use crate::state::world::WorldState;
use crate::state::Message;
use crate::ui_element::iced::draw_element;
use crate::World;
use iced::Container;
use iced_wgpu::Renderer;
use iced_winit::pane_grid;
use iced_winit::Command;
use iced_winit::Element;
use iced_winit::Length;
use iced_winit::PaneGrid;
use iced_winit::Program;
use iced_winit::Row;
use iced_winit::Text;

#[derive(PartialEq)]
enum PaneType {
    Entities,
    EntityEditor,
    Blank,
    None,
}

pub struct Panes {
    panes: pane_grid::State<PaneType>,
    focus: Option<pane_grid::Pane>,
    theme_state: &'static ThemeState,
}

#[derive(Debug, Clone)]
pub enum PanesMessage {
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
}

impl Panes {
    pub fn new(world: &World) -> Self {
        declare_global(WorldState::new(world));
        declare_global(ThemeState::default());
        declare_global(EntityState::default());

        let theme_state = use_global::<ThemeState>();

        Panes {
            panes: pane_grid::State::with_configuration(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.80,
                a: Box::new(pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.20,
                    a: Box::new(pane_grid::Configuration::Pane(PaneType::Entities)),
                    b: Box::new(pane_grid::Configuration::Split {
                        axis: pane_grid::Axis::Vertical,
                        ratio: 0.70,
                        a: Box::new(pane_grid::Configuration::Pane(PaneType::None)),
                        b: Box::new(pane_grid::Configuration::Pane(PaneType::EntityEditor)),
                    }),
                }),
                b: Box::new(pane_grid::Configuration::Pane(PaneType::Blank)),
            }),
            focus: None,
            theme_state,
        }
    }
}

impl Program for Panes {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        handle_message(message.clone());

        if let Message::Panes(message) = message {
            match message {
                PanesMessage::Clicked(pane) => self.focus = Some(pane),
                PanesMessage::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                    let pane1 = self.panes.get(&pane).unwrap();
                    let pane2 = self.panes.get(&target).unwrap();

                    // Avoid if one is a none pane
                    if PaneType::None != *pane1 && PaneType::None != *pane2 {
                        self.panes.swap(&pane, &target);
                    }
                }
                PanesMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                    self.panes.resize(&split, ratio)
                }
                _ => (),
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message, Renderer> {
        let theme_state = self.theme_state;

        let pane_grid: Element<Message, Renderer> = PaneGrid::new(&mut self.panes, |_id, pane| {
            let content: pane_grid::Content<Message, Renderer> = match pane {
                PaneType::Entities => {
                    let title =
                        Row::with_children(vec![Text::new("Entities").size(16).into()]).spacing(5);
                    let title_bar = pane_grid::TitleBar::new(title)
                        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
                        .padding(10)
                        .style(theme_state.theme);

                    let content =
                        Row::with_children(vec![draw_element(entity_list_component())]).padding(10);

                    pane_grid::Content::new(content)
                        .title_bar(title_bar)
                        .style(theme_state.theme.panel())
                        .into()
                }
                PaneType::EntityEditor => {
                    let title = Row::with_children(vec![Text::new("Edit entity").size(16).into()])
                        .spacing(5);

                    let title_bar = pane_grid::TitleBar::new(title)
                        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
                        .padding(10)
                        .style(theme_state.theme);

                    let content = Container::new(
                        Row::with_children(vec![draw_element(entity_edit_component())]).padding(10),
                    );

                    pane_grid::Content::new(content)
                        .title_bar(title_bar)
                        .style(theme_state.theme.panel())
                        .into()
                }
                PaneType::Blank => {
                    let title =
                        Row::with_children(vec![Text::new("Unknown").size(16).into()]).spacing(5);

                    let title_bar = pane_grid::TitleBar::new(title)
                        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
                        .padding(10)
                        .style(theme_state.theme);

                    let content = Row::with_children(vec![]).padding(10);

                    pane_grid::Content::new(content)
                        .title_bar(title_bar)
                        .style(theme_state.theme.panel())
                        .into()
                }
                PaneType::None => pane_grid::Content::new(Row::new()).into(),
            };

            content
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_click(|event| Message::Panes(PanesMessage::Clicked(event)))
        .on_drag(|event| Message::Panes(PanesMessage::Dragged(event)))
        .on_resize(10, |event| Message::Panes(PanesMessage::Resized(event)))
        .into();

        pane_grid.into()
    }
}
