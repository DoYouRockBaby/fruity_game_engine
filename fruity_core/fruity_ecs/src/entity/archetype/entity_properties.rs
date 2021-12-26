use crate::entity::archetype::EntityId;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::Deserialize;
use fruity_core::serialize::Serialize;
use maplit::hashmap;

/// This store all the information that are common accross all entities
#[derive(Debug, Clone)]
pub struct EntityProperties {
    /// The entity id
    pub entity_id: EntityId,

    /// the entity name
    pub name: String,

    /// If false, the entity will be ignored by the systems
    pub enabled: bool,
}

impl Serialize for EntityProperties {
    fn serialize(&self) -> Option<Serialized> {
        let serialized_object = Serialized::SerializedObject {
            class_name: "EntityProperties".to_string(),
            fields: hashmap! {
                "entity_id".to_string() => Serialized::U64(self.entity_id),
                "name".to_string() => Serialized::String(self.name.clone()),
                "enabled".to_string() => Serialized::Bool(self.enabled),
            },
        };

        Some(serialized_object)
    }
}

impl Deserialize for EntityProperties {
    type Output = EntityProperties;

    fn deserialize(
        serialized: &Serialized,
        object_factory: &ObjectFactoryService,
    ) -> Option<<Self>::Output> {
        let native_serialized = serialized.deserialize_native_objects(object_factory);
        if let Serialized::SerializedObject { class_name, fields } = native_serialized {
            if class_name == "EntityProperties" {
                Some(EntityProperties {
                    entity_id: fields.get("entity_id").map(|entity_id| {
                        if let Serialized::U64(entity_id) = entity_id {
                            Some(*entity_id)
                        } else {
                            None
                        }
                    })??,
                    name: fields.get("name").map(|name| {
                        if let Serialized::String(name) = name {
                            Some(name.clone())
                        } else {
                            None
                        }
                    })??,
                    enabled: fields.get("enabled").map(|enabled| {
                        if let Serialized::Bool(enabled) = enabled {
                            Some(*enabled)
                        } else {
                            None
                        }
                    })??,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
