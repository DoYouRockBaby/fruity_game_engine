use crate::entity::archetype::Archetype;
use crate::entity::entity_query::serialized::SerializedQueryParam;
use crate::entity::entity_reference::EntityReference;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::serialize::serialized::Serialized;

#[derive(FruityAny, Clone)]
pub struct WithEntity {}

impl SerializedQueryParam for WithEntity {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        vec![entity_reference.fruity_into()]
    }
}

#[derive(FruityAny, Clone)]
pub struct WithId {}

impl SerializedQueryParam for WithId {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        let entity_reader = entity_reference.read();
        vec![entity_reader.get_entity_id().fruity_into()]
    }
}

#[derive(FruityAny, Clone)]
pub struct WithName {}

impl SerializedQueryParam for WithName {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        let entity_reader = entity_reference.read();
        vec![entity_reader.get_name().fruity_into()]
    }
}

#[derive(FruityAny, Clone)]
pub struct WithEnabled {}

impl SerializedQueryParam for WithEnabled {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        let entity_reader = entity_reference.read();
        vec![entity_reader.is_enabled().fruity_into()]
    }
}

#[derive(FruityAny, Clone)]
pub struct With {
    pub identifier: String,
}

impl SerializedQueryParam for With {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, archetype: &Archetype) -> bool {
        archetype.identifier.contains(&self.identifier)
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        entity_reference
            .get_components_by_type_identifier(&self.identifier)
            .into_iter()
            .map(|component| component.fruity_into())
            .collect::<Vec<_>>()
    }
}

#[derive(FruityAny, Clone)]
pub struct WithOptional {
    pub identifier: String,
}

impl SerializedQueryParam for WithOptional {
    fn duplicate(&self) -> Box<dyn SerializedQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(&self, entity_reference: EntityReference) -> Vec<Serialized> {
        let components = entity_reference
            .get_components_by_type_identifier(&self.identifier)
            .into_iter()
            .map(|component| component.fruity_into())
            .collect::<Vec<_>>();

        if components.len() > 0 {
            components
        } else {
            vec![Serialized::Null]
        }
    }
}
