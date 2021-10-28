use crate::state::Message;
use crate::state::State;
use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_core::entity::entity_manager::EntityManager;
use iced::scrollable;
use iced::Checkbox;
use iced::Column;
use iced::Container;
use iced::Element;
use iced::Length;
use iced::Scrollable;

#[derive(Default)]
pub struct EntityList {
    scroll: scrollable::State,
}

impl EntityList {
    pub fn update(&mut self, message: Message) {}

    pub fn view(&mut self, state: &State) -> Element<Message> {
        let entity_cells: Element<_> = self
            .get_entities(&state)
            .enumerate()
            .fold(Column::new().spacing(20), |column, (i, entity)| {
                let checkbox =
                    Checkbox::new(entity.enabled, format!("entity {}", i), |_| Message::Empty)
                        .width(Length::Fill);

                column.push(checkbox)
            })
            .into();

        let content = Column::new().max_width(800).spacing(20).push(entity_cells);

        Scrollable::new(&mut self.scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }

    fn get_entities(&mut self, state: &State) -> impl Iterator<Item = EntitySharedRwLock> {
        let service_manager = state.world.service_manager.clone();
        let service_manager = service_manager.read().unwrap();
        let entity_manager = service_manager.read::<EntityManager>();
        entity_manager.iter_all_entities()
    }
}
