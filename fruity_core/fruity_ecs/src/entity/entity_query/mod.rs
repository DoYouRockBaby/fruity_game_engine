use crate::entity::archetype::Archetype;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use crate::entity::entity_reference::EntityReference;
use crate::EntityService;
use fruity_core::inject::Injectable;
use fruity_core::resource::resource_container::ResourceContainer;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;

pub mod union;
pub mod with;
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

pub trait QueryParam<'a> {
    type Item: Clone;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>>;

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

pub struct Query<T> {
    pub(crate) archetypes: Arc<RwLock<Vec<Arc<Archetype>>>>,
    pub(crate) _param_phantom: PhantomData<T>,
}

impl<'a, T: QueryParam<'a>> Query<T> {
    pub fn for_each(&self, callback: impl Fn(T::Item) + Send + Sync) {
        let archetypes = self.archetypes.read().unwrap();
        let archetypes = unsafe {
            std::mem::transmute::<&Vec<Arc<Archetype>>, &Vec<Arc<Archetype>>>(&archetypes)
        };

        let archetype_iter = T::filter_archetype(Box::new(
            archetypes.iter().map(|archetype| archetype.clone()),
        ));

        let entities = archetype_iter
            .map(|archetype| {
                let archetype = archetype.clone();
                archetype.iter()
            })
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
