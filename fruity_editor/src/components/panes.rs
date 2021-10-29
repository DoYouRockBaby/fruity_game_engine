use crate::components::entity::entity_editor::EntityEditor;
use crate::components::entity::entity_list::EntityList;
use crate::state::update_state;
use crate::state::Message;
use crate::state::State;
use crate::World;
use iced_wgpu::Renderer;
use iced_winit::pane_grid;
use iced_winit::Command;
use iced_winit::Element;
use iced_winit::Length;
use iced_winit::PaneGrid;
use iced_winit::Program;
use iced_winit::Row;
use iced_winit::Text;

enum PaneType {
    Entities(EntityList),
    EntityEditor(EntityEditor),
    None,
}

pub struct Panes {
    panes: pane_grid::State<PaneType>,
    focus: Option<pane_grid::Pane>,
    state: State,
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
        let state = State::new(world);

        Panes {
            panes: pane_grid::State::with_configuration(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.10,
                a: Box::new(pane_grid::Configuration::Pane(PaneType::Entities(
                    EntityList::new(&state),
                ))),
                b: Box::new(pane_grid::Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.10,
                    a: Box::new(pane_grid::Configuration::Pane(PaneType::None)),
                    b: Box::new(pane_grid::Configuration::Pane(PaneType::EntityEditor(
                        EntityEditor::new(),
                    ))),
                }),
            }),
            focus: None,
            state,
        }
    }
}

impl Program for Panes {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        update_state(&mut self.state, message.clone());
        //self.entity_list.update(message.clone());

        if let Message::Panes(message) = message {
            match message {
                PanesMessage::Clicked(pane) => self.focus = Some(pane),
                PanesMessage::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                    self.panes.swap(&pane, &target)
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
        let state = &self.state;
        let panes = &mut self.panes;

        let pane_grid: Element<Message, Renderer> = PaneGrid::new(panes, |_id, pane| {
            let content: pane_grid::Content<Message, Renderer> = match pane {
                PaneType::Entities(entity_list) => {
                    let title =
                        Row::with_children(vec![Text::new("Entities").size(16).into()]).spacing(5);
                    let title_bar = pane_grid::TitleBar::new(title)
                        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
                        .padding(10)
                        .style(state.theme.theme);

                    pane_grid::Content::new(entity_list.view(state))
                        .title_bar(title_bar)
                        .style(state.theme.theme)
                        .into()
                }
                PaneType::EntityEditor(entity_editor) => {
                    let title = Row::with_children(vec![Text::new("Edit entity").size(16).into()])
                        .spacing(5);
                    let title_bar = pane_grid::TitleBar::new(title)
                        //.panes(pane.panes.view(id, total_panes, pane.is_pinned))
                        .padding(10)
                        .style(state.theme.theme);

                    pane_grid::Content::new(entity_editor.view(state))
                        .title_bar(title_bar)
                        .style(state.theme.theme)
                        .into()
                }
                PaneType::None => pane_grid::Content::new(Row::new()).into(),
            };

            content
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_click(|event| Message::Panes(PanesMessage::Clicked(event)))
        .on_drag(|event| Message::Panes(PanesMessage::Dragged(event)))
        .on_resize(10, |event| Message::Panes(PanesMessage::Resized(event)))
        .into();

        pane_grid.into()
    }
}
