use crate::entity::archetype::Component;
use crate::entity::archetype::ComponentDecoder;
use crate::entity::archetype::EntityId;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::Deserialize;
use fruity_core::signal::Signal;
use fruity_core::utils::slice::copy;
use std::any::TypeId;
use std::sync::Arc;

/// This store all the information that are common accross all entities
#[derive(Debug, Clone, FruityAny)]
pub struct Entity {
    /// The entity id
    pub entity_id: EntityId,

    /// the entity name
    pub name: String,

    /// If false, the entity will be ignored by the systems
    pub enabled: bool,

    /// A marker for an entity that is deleted but that is not yet free into memory
    pub deleted: bool,

    /// A signal that is sent when the a write lock on the entity is released
    pub on_deleted: Signal<()>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            entity_id: EntityId::default(),
            name: String::default(),
            enabled: true,
            deleted: false,
            on_deleted: Signal::default(),
        }
    }
}

impl Component for Entity {
    fn encode_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }

    fn encode(&self, buffer: &mut [u8]) {
        let encoded = unsafe {
            std::slice::from_raw_parts(
                (&*self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        };

        copy(buffer, encoded);
    }

    fn get_decoder(&self) -> ComponentDecoder {
        |data| {
            let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
            &body[0]
        }
    }

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl IntrospectObject for Entity {
    fn get_class_name(&self) -> String {
        "Entity".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "entity_id".to_string(),
                ty: TypeId::of::<EntityId>(),
                serializable: true,
                getter: Arc::new(|this| {
                    this.downcast_ref::<Entity>()
                        .unwrap()
                        .entity_id
                        .clone()
                        .fruity_into()
                }),
                setter: SetterCaller::Mut(Arc::new(|this, value| {
                    let this = this.downcast_mut::<Entity>().unwrap();
                    match EntityId::fruity_try_from(value) {
                        Ok(value) => this.entity_id = value,
                        Err(_) => {
                            log::error!("Expected a {} for property {:?}", "EntityId", "entity_id",);
                        }
                    }
                })),
            },
            FieldInfo {
                name: "name".to_string(),
                ty: TypeId::of::<String>(),
                serializable: true,
                getter: Arc::new(|this| {
                    this.downcast_ref::<Entity>()
                        .unwrap()
                        .name
                        .clone()
                        .fruity_into()
                }),
                setter: SetterCaller::Mut(Arc::new(|this, value| {
                    let this = this.downcast_mut::<Entity>().unwrap();
                    match String::fruity_try_from(value) {
                        Ok(value) => this.name = value,
                        Err(_) => {
                            log::error!("Expected a {} for property {:?}", "String", "name",);
                        }
                    }
                })),
            },
            FieldInfo {
                name: "enabled".to_string(),
                ty: TypeId::of::<bool>(),
                serializable: true,
                getter: Arc::new(|this| {
                    this.downcast_ref::<Entity>()
                        .unwrap()
                        .enabled
                        .clone()
                        .fruity_into()
                }),
                setter: SetterCaller::Mut(Arc::new(|this, value| {
                    let this = this.downcast_mut::<Entity>().unwrap();
                    match bool::fruity_try_from(value) {
                        Ok(value) => this.enabled = value,
                        Err(_) => {
                            log::error!("Expected a {} for property {:?}", "bool", "enabled",);
                        }
                    }
                })),
            },
            FieldInfo {
                name: "deleted".to_string(),
                ty: TypeId::of::<bool>(),
                serializable: false,
                getter: Arc::new(|this| {
                    this.downcast_ref::<Entity>()
                        .unwrap()
                        .deleted
                        .clone()
                        .fruity_into()
                }),
                setter: SetterCaller::Mut(Arc::new(|this, value| {
                    let this = this.downcast_mut::<Entity>().unwrap();
                    match bool::fruity_try_from(value) {
                        Ok(value) => this.deleted = value,
                        Err(_) => {
                            log::error!("Expected a {} for property {:?}", "bool", "deleted",);
                        }
                    }
                })),
            },
            FieldInfo {
                name: "on_deleted".to_string(),
                ty: TypeId::of::<Signal<()>>(),
                serializable: false,
                getter: Arc::new(|this| {
                    this.downcast_ref::<Entity>()
                        .unwrap()
                        .on_deleted
                        .clone()
                        .fruity_into()
                }),
                setter: SetterCaller::None,
            },
        ]
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }
}

impl Deserialize for Entity {
    type Output = Entity;

    fn deserialize(
        serialized: &Serialized,
        object_factory: &ObjectFactoryService,
    ) -> Option<<Self>::Output> {
        let native_serialized = serialized.deserialize_native_objects(object_factory);
        if let Serialized::SerializedObject { class_name, fields } = native_serialized {
            if class_name == "Entity" {
                Some(Entity {
                    name: fields
                        .get("name")
                        .map(|name| {
                            if let Serialized::String(name) = name {
                                name.clone()
                            } else {
                                "Unknown".to_string()
                            }
                        })
                        .unwrap_or("Unknown".to_string()),
                    enabled: fields
                        .get("enabled")
                        .map(|enabled| {
                            if let Serialized::Bool(enabled) = enabled {
                                enabled.clone()
                            } else {
                                true
                            }
                        })
                        .unwrap_or(true),
                    ..Default::default()
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
