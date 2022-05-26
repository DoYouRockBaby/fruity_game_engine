use crate::entity::archetype::Archetype;
use crate::entity::entity_query::QueryParam;
use crate::entity::entity_query::RequestedEntityGuard;
use crate::entity::entity_reference::EntityReference;

macro_rules! tuple_impl_generics {
    ($t1:ident, $($tn:ident),+) => {
    #[allow(unused_parens)]
    #[allow(non_snake_case)]
    impl<
                'a,
                $t1: QueryParam<'a> + 'static,
                $ ($tn: QueryParam<'a> + 'static),*
            > QueryParam<'a> for ($t1, $ ($tn),*)
        {
            type Item = (
                $t1::Item,
                $ ($tn::Item),*
            );


            fn filter_archetype(archetype: &Archetype) -> bool {
                <($ ($tn),*)>::filter_archetype(archetype) && $t1::filter_archetype(archetype)
            }

            fn require_read() -> bool {
                $t1::require_read() || $ ($tn::require_read())||*
            }

            fn require_write() -> bool {
                $t1::require_write() || $ ($tn::require_write())||*
            }

            fn iter_entity_components(
                entity_reference: EntityReference,
                entity_guard: &'a RequestedEntityGuard<'a>,
            ) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                Box::new(
                    <($ ($tn),*)>::iter_entity_components(entity_reference.clone(), entity_guard)
                        .map(move |($ ($tn),*)| {
                            $t1::iter_entity_components(entity_reference.clone(), entity_guard)
                                .map(move |$t1| ($t1, $ ($tn.clone()),*))
                        })
                        .flatten(),
                )
            }
        }
    };
}

tuple_impl_generics!(T1, T2);
tuple_impl_generics!(T1, T2, T3);
tuple_impl_generics!(T1, T2, T3, T4);
tuple_impl_generics!(T1, T2, T3, T4, T5);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
tuple_impl_generics!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
tuple_impl_generics!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
tuple_impl_generics!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
tuple_impl_generics!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
