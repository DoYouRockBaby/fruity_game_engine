use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::archetype::Archetype;
use crate::entity::entity::EntityId;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_query::RequestedEntityGuard;
use crate::entity::entity_reference::EntityReference;
use std::marker::PhantomData;
use std::sync::Arc;

/// The entity reference
pub struct WithEntity;

impl<'a> QueryParam<'a> for WithEntity {
    type Item = EntityReference;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        entity_reference: EntityReference,
        _entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        Box::new(vec![entity_reference.clone()].into_iter())
    }
}

/// The entity id
pub struct WithId;

impl<'a> QueryParam<'a> for WithId {
    type Item = EntityId;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        entity_reference: EntityReference,
        _entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        let entity_id = {
            let entity_reader = entity_reference.read();
            entity_reader.get_entity_id()
        };

        Box::new(vec![entity_id].into_iter())
    }
}

/// The entity name
pub struct WithName;

impl<'a> QueryParam<'a> for WithName {
    type Item = String;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        entity_reference: EntityReference,
        _entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        let name = {
            let entity_reader = entity_reference.read();
            entity_reader.get_name()
        };

        Box::new(vec![name].into_iter())
    }
}

/// Is entity enabled
pub struct WithEnabled;

impl<'a> QueryParam<'a> for WithEnabled {
    type Item = bool;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        entity_reference: EntityReference,
        _entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        let enabled = {
            let entity_reader = entity_reference.read();
            entity_reader.is_enabled()
        };

        Box::new(vec![enabled].into_iter())
    }
}

/// A readable component reference
pub struct With<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for With<T> {
    type Item = TypedComponentReadGuard<'a, T>;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        Box::new(iter.filter(|archetype| archetype.identifier.contains(&T::get_component_name())))
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        _entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self::Item>>,
                        Box<dyn Iterator<Item = Self::Item>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self::Item>>,
                        Box<dyn Iterator<Item = Self::Item>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::None => panic!(),
        }
    }
}

/// A writable component reference
pub struct WithMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithMut<T> {
    type Item = TypedComponentWriteGuard<'a, T>;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        Box::new(iter.filter(|archetype| archetype.identifier.contains(&T::get_component_name())))
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn iter_entity_components(
        _entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        match entity_guard {
            RequestedEntityGuard::Read(_) => panic!(),
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components_mut::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self::Item>>,
                        Box<dyn Iterator<Item = Self::Item>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::None => panic!(),
        }
    }
}

/// A readable optional component reference
pub struct WithOptional<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithOptional<T> {
    type Item = Option<TypedComponentReadGuard<'a, T>>;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        true
    }

    fn require_write() -> bool {
        false
    }

    fn iter_entity_components(
        _entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => {
                let iter = entity_guard.iter_components::<T>().peekable();
                let mut iter = iter.peekable();

                match iter.peek() {
                    Some(_) => unsafe {
                        std::mem::transmute::<
                            Box<dyn Iterator<Item = Self::Item>>,
                            Box<dyn Iterator<Item = Self::Item>>,
                        >(Box::new(iter.map(|elem| Some(elem))))
                    },
                    None => Box::new(vec![None].into_iter()),
                }
            }
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = entity_guard.iter_components::<T>().peekable();
                let mut iter = iter.peekable();

                match iter.peek() {
                    Some(_) => unsafe {
                        std::mem::transmute::<
                            Box<dyn Iterator<Item = Self::Item>>,
                            Box<dyn Iterator<Item = Self::Item>>,
                        >(Box::new(iter.map(|elem| Some(elem))))
                    },
                    None => Box::new(vec![None].into_iter()),
                }
            }
            RequestedEntityGuard::None => Box::new(vec![None].into_iter()),
        }
    }
}

/// A writable optional component reference
pub struct WithOptionMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent + 'static> QueryParam<'a> for WithOptionMut<T> {
    type Item = Option<TypedComponentWriteGuard<'a, T>>;

    fn filter_archetype(
        iter: Box<dyn Iterator<Item = Arc<Archetype>>>,
    ) -> Box<dyn Iterator<Item = Arc<Archetype>>> {
        iter
    }

    fn require_read() -> bool {
        false
    }

    fn require_write() -> bool {
        true
    }

    fn iter_entity_components(
        _entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        match entity_guard {
            RequestedEntityGuard::Read(_) => Box::new(vec![None].into_iter()),
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = entity_guard.iter_components_mut::<T>().peekable();
                let mut iter = iter.peekable();

                match iter.peek() {
                    Some(_) => unsafe {
                        std::mem::transmute::<
                            Box<dyn Iterator<Item = Self::Item>>,
                            Box<dyn Iterator<Item = Self::Item>>,
                        >(Box::new(iter.map(|elem| Some(elem))))
                    },
                    None => Box::new(vec![None].into_iter()),
                }
            }
            RequestedEntityGuard::None => Box::new(vec![None].into_iter()),
        }
    }
}
