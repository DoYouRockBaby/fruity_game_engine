use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::archetype::Archetype;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_query::RequestedEntityGuard;
use crate::entity::entity_reference::EntityReference;
use std::marker::PhantomData;
use std::sync::Arc;

/// A readable component reference
pub type With<'s, T> = TypedComponentReadGuard<'s, T>;

/// A writable component reference
pub type WithMut<'s, T> = TypedComponentWriteGuard<'s, T>;

/// A readable optional component reference
pub type WithOption<'s, T> = Option<With<'s, T>>;

/// A writable optional component reference
pub type WithOptionMut<'s, T> = Option<WithMut<'s, T>>;

/// The entity reference
pub struct WithEntity;

/// The entity id
pub struct WithId;

/// The entity name
pub struct WithName;

/// Is entity enabled
pub struct WithEnabled;

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
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        Box::new(vec![entity_reference.clone()].into_iter())
    }
}

impl<'a, T: Component + StaticComponent> QueryParam<'a> for With<'a, T> {
    type Item = Self;

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
        entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self>>,
                        Box<dyn Iterator<Item = Self>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self>>,
                        Box<dyn Iterator<Item = Self>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl<'a, T: Component + StaticComponent> QueryParam<'a> for WithMut<'a, T> {
    type Item = Self;

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
        entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item>> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => panic!(),
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = Box::new(entity_guard.iter_components_mut::<T>());
                unsafe {
                    std::mem::transmute::<
                        Box<dyn Iterator<Item = Self>>,
                        Box<dyn Iterator<Item = Self>>,
                    >(iter)
                }
            }
            RequestedEntityGuard::None => panic!(),
        }
    }
}

impl<'a, T: Component + StaticComponent> QueryParam<'a> for WithOption<'a, T> {
    type Item = Self;

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
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => {
                let iter = entity_guard.iter_components::<T>().peekable();
                let mut iter = iter.peekable();

                match iter.peek() {
                    Some(_) => unsafe {
                        std::mem::transmute::<
                            Box<dyn Iterator<Item = Self>>,
                            Box<dyn Iterator<Item = Self>>,
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
                            Box<dyn Iterator<Item = Self>>,
                            Box<dyn Iterator<Item = Self>>,
                        >(Box::new(iter.map(|elem| Some(elem))))
                    },
                    None => Box::new(vec![None].into_iter()),
                }
            }
            RequestedEntityGuard::None => Box::new(vec![None].into_iter()),
        }
    }
}

pub struct WithOptionalComponentMut<T> {
    _phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent> QueryParam<'a> for WithOptionMut<'a, T> {
    type Item = Self;

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
        entity_reference: EntityReference,
        entity_guard: &'a RequestedEntityGuard<'a>,
    ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        match entity_guard {
            RequestedEntityGuard::Read(entity_guard) => Box::new(vec![None].into_iter()),
            RequestedEntityGuard::Write(entity_guard) => {
                let iter = entity_guard.iter_components_mut::<T>().peekable();
                let mut iter = iter.peekable();

                match iter.peek() {
                    Some(_) => unsafe {
                        std::mem::transmute::<
                            Box<dyn Iterator<Item = Self>>,
                            Box<dyn Iterator<Item = Self>>,
                        >(Box::new(iter.map(|elem| Some(elem))))
                    },
                    None => Box::new(vec![None].into_iter()),
                }
            }
            RequestedEntityGuard::None => Box::new(vec![None].into_iter()),
        }
    }
}
