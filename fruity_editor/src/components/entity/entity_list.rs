use crate::state::entity::EntityMessage;
use crate::state::Message;
use crate::state::State;
use fruity_core::entity::archetype::rwlock::EntitySharedRwLock;
use fruity_core::entity::entity::EntityId;
use fruity_core::entity::entity_manager::EntityManager;
use iced_wgpu::Renderer;
use iced_winit::button;
use iced_winit::scrollable;
use iced_winit::Button;
use iced_winit::Element;
use iced_winit::Length;
use iced_winit::Scrollable;
use iced_winit::Text;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;

pub struct EntityItem {
    entity: EntitySharedRwLock,
    button_state: button::State,
}

pub struct EntityList {
    scroll: scrollable::State,
    items: Arc<Mutex<HashMap<EntityId, EntityItem>>>,
}

impl EntityList {
    pub fn new(state: &State) -> EntityList {
        let service_manager = state.world.service_manager.clone();
        let service_manager = service_manager.read().unwrap();
        let entity_manager = service_manager.read::<EntityManager>();

        let items = Arc::new(Mutex::new(
            entity_manager
                .iter_all_entities()
                .map(|entity| {
                    (
                        entity.entity_id,
                        EntityItem {
                            entity,
                            button_state: button::State::new(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>(),
        ));

        let items_2 = items.clone();
        entity_manager
            .on_entity_created
            .add_observer(move |entity| {
                let mut items = items_2.lock().unwrap();
                items.insert(
                    entity.entity_id,
                    EntityItem {
                        entity: entity.clone(),
                        button_state: button::State::new(),
                    },
                );
            });

        let items_2 = items.clone();
        entity_manager
            .on_entity_removed
            .add_observer(move |entity_id| {
                let mut items = items_2.lock().unwrap();
                items.remove(entity_id);
            });

        EntityList {
            scroll: scrollable::State::default(),
            items,
        }
    }

    pub fn update(&mut self, message: &Message) {
        let mut items = self.items.lock().unwrap();
        items.iter_mut().for_each(|item| item.1.update(message))
    }

    pub fn view(&mut self, state: &State) -> Element<Message, Renderer> {
        let mut items = self.items.lock().unwrap();

        // TODO: Try to find a way to remove that
        let items = unsafe {
            std::mem::transmute::<
                &mut HashMap<EntityId, EntityItem>,
                &mut HashMap<EntityId, EntityItem>,
            >(items.deref_mut())
        };

        items
            .iter_mut()
            .fold(
                Scrollable::new(&mut self.scroll)
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Units(500))
                    .style(state.theme.theme),
                |scrollable, (_, item)| scrollable.push(item.view(state)),
            )
            .into()
    }
}

impl EntityItem {
    pub fn update(&mut self, _message: &Message) {}

    pub fn view(&mut self, state: &State) -> Element<Message, Renderer> {
        let text = Text::new(&self.entity.name).size(16);

        Button::new(&mut self.button_state, text)
            .style(state.theme.theme.list_item())
            .on_press(Message::Entity(EntityMessage::SelectEntity(
                self.entity.clone(),
            )))
            .width(Length::Fill)
            .into()
    }
}
