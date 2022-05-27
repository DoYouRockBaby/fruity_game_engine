use crate::entity::archetype::Archetype;
use crate::entity::archetype::ArchetypeArcRwLock;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use crate::EntityService;
use fruity_core::inject::Injectable;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::RwLock;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::marker::PhantomData;
use std::sync::Arc;

/// Queries for scripting languages
pub(crate) mod serialized;

/// Queries for tuples
pub mod tuple;

/// Queries for with stuffs
pub mod with;

/// Queries for without stuffs
pub mod without;

/// An enum to pass a guard into the [’QueryInjectable’]
#[derive(Clone)]
pub enum RequestedEntityGuard<'a> {
    /// No guard required
    None,
    /// Read guard required
    Read(EntityReadGuard<'a>),
    /// Write guard required
    Write(EntityWriteGuard<'a>),
}

/// A trait that should be implement for everything that can be queried from ['EntityService']
pub trait QueryParam<'a> {
    /// The type of the query callback parameter
    type Item: Clone;

    /// A filter over the archetypes
    fn filter_archetype(archetype: &Archetype) -> bool;

    /// Does this require a read guard over the reference
    fn require_read() -> bool;

    /// Does this require a write guard over the reference
    fn require_write() -> bool;

    /// Iter over the queried components into a given entity
    fn iter_entity_components(
        entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

/// A query over entities
pub struct Query<T> {
    pub(crate) archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
    pub(crate) _param_phantom: PhantomData<T>,
}

impl<T> Clone for Query<T> {
    fn clone(&self) -> Self {
        Query {
            archetypes: self.archetypes.clone(),
            _param_phantom: PhantomData {},
        }
    }
}

// Safe cause all non phantom fields implements ['Sync']
unsafe impl<T> Sync for Query<T> {}

// Safe cause all non phantom fields implements ['Send']
unsafe impl<T> Send for Query<T> {}

impl<'a, T: QueryParam<'a> + 'static> Query<T> {
    /// Call a function for every entities of an query
    pub fn for_each(&self, callback: impl Fn(T::Item) + Send + Sync) {
        let archetypes = self.archetypes.read();
        let archetype_iter = archetypes
            .iter()
            .filter(|archetype| T::filter_archetype(&archetype.read()));

        let entities = archetype_iter
            .map(|archetype| archetype.iter(false))
            .flatten()
            .collect::<Vec<_>>();

        entities.into_iter().par_bridge().for_each(|entity| {
            let entity_guard = if T::require_write() {
                RequestedEntityGuard::Write(entity.write())
            } else if T::require_read() {
                RequestedEntityGuard::Read(entity.read())
            } else {
                RequestedEntityGuard::None
            };

            // TODO: Find a way to remove it
            let entity_guard = unsafe {
                std::mem::transmute::<&RequestedEntityGuard, &RequestedEntityGuard>(&entity_guard)
            };

            T::iter_entity_components(entity.clone(), &entity_guard)
                .for_each(|param| callback(param))
        });
    }
}

impl<'a, T: QueryParam<'a> + 'static> Injectable for Query<T> {
    fn from_resource_container(resource_container: &Arc<ResourceContainer>) -> Self {
        let entity_service = resource_container.require::<EntityService>();
        let entity_service = entity_service.read();

        entity_service.query::<T>()
    }
}
