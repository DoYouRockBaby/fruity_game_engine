use crate::entity::archetype::AnyComponent;
use crate::entity::archetype::Component;

/// A abstraction of a collection over components
pub trait ComponentCollection: Sync + Send {
    /// Get a single component by index
    fn get(&self, index: &usize) -> Option<&dyn Component>;

    /// Add components to the collection
    ///
    /// # Arguments
    /// * `components` - The components that will be added
    ///
    fn add_many(&mut self, components: Vec<AnyComponent>);

    /// Remove components from the collection
    ///
    /// # Arguments
    /// * `index` - The index of the first component to remove
    /// * `count` - The number of components that will be removed
    ///
    fn remove_many(&mut self, index: usize, count: usize) -> Vec<AnyComponent>;
}
