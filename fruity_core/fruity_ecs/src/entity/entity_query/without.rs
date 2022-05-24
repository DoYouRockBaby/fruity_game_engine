use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::entity::archetype::Archetype;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_query::RequestedEntityGuard;
use crate::entity::entity_reference::EntityReference;
use std::marker::PhantomData;
use std::sync::Arc;

/// Exclude a component from a query
pub struct Without<T: Component + StaticComponent + 'static> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for Without<T> {
    type Item = ();

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        Box::new(iter.filter(|archetype| !archetype.identifier.contains(&T::get_component_name())))
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        _entity_reference: EntityReference,
        _entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(vec![()].into_iter())
    }
}
