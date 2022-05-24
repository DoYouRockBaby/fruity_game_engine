use crate::entity::archetype::Archetype;
use crate::entity::entity_query::serialized::params::With;
use crate::entity::entity_query::serialized::params::WithEnabled;
use crate::entity::entity_query::serialized::params::WithId;
use crate::entity::entity_query::serialized::params::WithName;
use crate::entity::entity_query::serialized::params::WithOptional;
use crate::entity::entity_reference::EntityReference;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::serialize::serialized::Callback;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::cast_introspect_mut;
use fruity_core::utils::introspect::ArgumentCaster;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

pub(crate) mod params;

pub trait SerializedQueryParam: FruityAny {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam>;
    fn filter_archetype(&self, archetype: &Archetype) -> bool;
    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized>;
}

#[derive(FruityAny)]
pub struct SerializedQuery {
    pub archetypes: Arc<RwLock<Vec<Arc<Archetype>>>>,
    pub params: Vec<Box<dyn SerializedQueryParam>>,
}

impl Clone for SerializedQuery {
    fn clone(&self) -> Self {
        Self {
            archetypes: self.archetypes.clone(),
            params: self
                .params
                .iter()
                .map(|param| param.duplicate())
                .collect::<Vec<_>>(),
        }
    }
}

impl Debug for SerializedQuery {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl SerializedQuery {
    pub fn with_id(&mut self) {
        self.params.push(Box::new(WithId {}));
    }

    pub fn with_name(&mut self) {
        self.params.push(Box::new(WithName {}));
    }

    pub fn with_enabled(&mut self) {
        self.params.push(Box::new(WithEnabled {}));
    }

    pub fn with(&mut self, component_identifier: &str) {
        self.params.push(Box::new(With {
            identifier: component_identifier.to_string(),
        }));
    }

    pub fn with_optional(&mut self, component_identifier: &str) {
        self.params.push(Box::new(WithOptional {
            identifier: component_identifier.to_string(),
        }));
    }

    pub fn for_each(&self, callback: impl Fn(&[Serialized]) + Send + Sync) {
        let archetypes = self.archetypes.read().unwrap();
        let archetypes = unsafe {
            std::mem::transmute::<&Vec<Arc<Archetype>>, &Vec<Arc<Archetype>>>(&archetypes)
        };

        let mut archetype_iter: Box<dyn Iterator<Item = Arc<Archetype>>> =
            Box::new(archetypes.iter().map(|archetype| archetype.clone()));
        for param in self.params.iter() {
            archetype_iter =
                Box::new(archetype_iter.filter(|archetype| param.filter_archetype(archetype)));
        }

        let entities = archetype_iter
            .map(|archetype| {
                let archetype = archetype.clone();
                archetype.iter()
            })
            .flatten()
            .collect::<Vec<_>>();

        entities
            .into_iter() /*.par_bridge()*/
            .for_each(|entity| {
                let serialized_params = self
                    .params
                    .iter()
                    .map(|param| param.get_entity_components(entity.clone()))
                    .multi_cartesian_product();

                serialized_params.for_each(|params| callback(&params))
            });
    }
}

impl FruityInto<Serialized> for SerializedQuery {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

impl SerializableObject for SerializedQuery {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl IntrospectObject for SerializedQuery {
    fn get_class_name(&self) -> String {
        "Query".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "with_id".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, _args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);
                    this.with_id();

                    Ok(Some(Serialized::NativeObject(this.duplicate())))
                })),
            },
            MethodInfo {
                name: "with_name".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, _args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);
                    this.with_name();

                    Ok(Some(Serialized::NativeObject(this.duplicate())))
                })),
            },
            MethodInfo {
                name: "with_enabled".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, _args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);
                    this.with_enabled();

                    Ok(Some(Serialized::NativeObject(this.duplicate())))
                })),
            },
            MethodInfo {
                name: "with".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);

                    let mut caster = ArgumentCaster::new("with", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.with(&arg1);

                    Ok(Some(Serialized::NativeObject(this.duplicate())))
                })),
            },
            MethodInfo {
                name: "with_optional".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);

                    let mut caster = ArgumentCaster::new("with_optional", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.with_optional(&arg1);

                    Ok(Some(Serialized::NativeObject(this.duplicate())))
                })),
            },
            MethodInfo {
                name: "for_each".to_string(),
                call: MethodCaller::Mut(Arc::new(|this, args| {
                    let this = cast_introspect_mut::<SerializedQuery>(this);

                    let mut caster = ArgumentCaster::new("for_each", args);
                    let arg1 = caster.cast_next::<Callback>()?;

                    let callback = arg1.callback;
                    this.for_each(|args| {
                        callback(args.to_vec()).ok();
                    });

                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
