use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;

pub trait ComponentCollection: Sync + Send {
    fn get(&self, index: &usize) -> Vec<&dyn Component>;
    fn add(&mut self, components: Vec<AnyComponent>);
    fn remove(&mut self, index: usize) -> Vec<AnyComponent>;
    fn get_components_per_entity(&self) -> usize;
}
