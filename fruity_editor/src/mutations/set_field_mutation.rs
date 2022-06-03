use crate::mutations::mutation::Mutation;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct SetFieldMutation {
    pub target: Box<dyn SerializableObject>,
    pub field: String,
    pub previous_value: Serialized,
    pub new_value: Serialized,
}

impl Mutation for SetFieldMutation {
    fn apply(&self) {
        if let Some(field_info) = self
            .target
            .get_field_infos()
            .into_iter()
            .find(|field_info| field_info.name == self.field)
        {
            // Modify the field value
            match &field_info.setter {
                SetterCaller::Const(call) => {
                    call(self.target.deref().as_any_ref(), self.new_value.clone())
                }
                SetterCaller::Mut(call) => {
                    let mut introspect_object = self.target.duplicate();
                    call(
                        introspect_object.deref_mut().as_any_mut(),
                        self.new_value.clone(),
                    )
                }
                SetterCaller::None => {}
            };
        }
    }

    fn undo(&self) {
        if let Some(field_info) = self
            .target
            .get_field_infos()
            .into_iter()
            .find(|field_info| field_info.name == self.field)
        {
            // Modify the field value from the old stored value
            match &field_info.setter {
                SetterCaller::Const(call) => call(
                    self.target.deref().as_any_ref(),
                    self.previous_value.clone(),
                ),
                SetterCaller::Mut(call) => {
                    let mut introspect_object = self.target.duplicate();
                    call(
                        introspect_object.deref_mut().as_any_mut(),
                        self.previous_value.clone(),
                    )
                }
                SetterCaller::None => {}
            };
        }
    }
}
