use crate::mutations::mutation::Mutation;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct MutationService {
    previous_mutations: Vec<Box<dyn Mutation>>,
    next_mutations: Vec<Box<dyn Mutation>>,
}

impl MutationService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> Self {
        Self {
            previous_mutations: Vec::new(),
            next_mutations: Vec::new(),
        }
    }

    pub fn push_action(&mut self, mutation: impl Mutation + 'static) {
        mutation.apply();

        self.next_mutations.clear();
        self.previous_mutations.push(Box::new(mutation));
    }

    pub fn undo(&mut self) {
        if let Some(mutation) = self.previous_mutations.pop() {
            mutation.undo();
            self.next_mutations.push(mutation);
        }
    }

    pub fn redo(&mut self) {
        if let Some(mutation) = self.next_mutations.pop() {
            mutation.apply();
            self.previous_mutations.push(mutation);
        }
    }

    pub fn can_undo(&self) -> bool {
        self.previous_mutations.len() > 0
    }

    pub fn can_redo(&self) -> bool {
        self.next_mutations.len() > 0
    }
}

impl Debug for MutationService {
    fn fmt(&self, _formatter: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

impl IntrospectObject for MutationService {
    fn get_class_name(&self) -> String {
        "MutationService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for MutationService {}
