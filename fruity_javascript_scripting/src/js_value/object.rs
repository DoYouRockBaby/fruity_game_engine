use crate::js_value::function::JsFunction;
use crate::js_value::property::JsProperty;
use crate::js_value::value::JsValue;
use crate::serialize::deserialize::deserialize_v8;
use crate::serialize::serialize::serialize_v8;
use core::ffi::c_void;
use fruity_core::service::Service;
use fruity_ecs::component::component::Component;
use fruity_introspect::MethodCaller;
use rusty_v8 as v8;
use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

pub struct ComponentRef(pub(crate) &'static dyn Component);

pub struct ComponentMut(pub(crate) &'static mut dyn Component);

enum JsObjectInternalObject {
    Service(Arc<RwLock<Box<dyn Service>>>),
    Component(ComponentRef),
    ComponentMut(ComponentMut),
    None,
}

pub struct JsObject {
    pub(crate) fields: HashMap<String, Box<dyn JsValue>>,
    internal_object: JsObjectInternalObject,
}

impl JsObject {
    pub fn new() -> JsObject {
        JsObject {
            fields: HashMap::new(),
            internal_object: JsObjectInternalObject::None,
        }
    }

    pub fn from_service(service: Arc<RwLock<Box<dyn Service>>>) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        let method_infos = {
            let reader = service.read().unwrap();
            reader.get_method_infos()
        };

        for method_info in method_infos {
            fields.insert(
                method_info.name,
                Box::new(JsFunction::new(service_callback)),
            );
        }

        JsObject {
            fields,
            internal_object: JsObjectInternalObject::Service(service),
        }
    }

    pub fn from_component(component: &dyn Component) -> JsObject {
        let component = unsafe { &*(component as *const _) } as &dyn Component;
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        let field_infos = component.get_field_infos();

        for field_info in field_infos {
            fields.insert(field_info.name, Box::new(JsProperty::new(false)));
        }

        JsObject {
            fields,
            internal_object: JsObjectInternalObject::Component(ComponentRef(component)),
        }
    }

    pub fn from_component_mut(component: &mut dyn Component) -> JsObject {
        let component = unsafe { &mut *(component as *mut _) } as &mut dyn Component;
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        let field_infos = component.get_field_infos();

        for field_info in field_infos {
            fields.insert(field_info.name, Box::new(JsProperty::new(true)));
        }

        JsObject {
            fields,
            internal_object: JsObjectInternalObject::ComponentMut(ComponentMut(component)),
        }
    }

    pub fn add_field<T: JsValue>(&mut self, name: &str, value: T) {
        self.fields.insert(name.to_string(), Box::new(value));
    }

    pub fn set_func(
        &mut self,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> &mut JsFunction {
        self.fields
            .insert(name.to_string(), Box::new(JsFunction::new(callback)));

        self.fields
            .get_mut(&name.to_string())
            .unwrap()
            .as_mut_any()
            .downcast_mut::<JsFunction>()
            .unwrap()
    }
}

impl JsValue for JsObject {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        // Create the object
        let object_template = v8::ObjectTemplate::new(scope);
        if let JsObjectInternalObject::Service(_) = self.internal_object {
            object_template.set_internal_field_count(1);
        } else if let JsObjectInternalObject::Component(_) = self.internal_object {
            object_template.set_internal_field_count(1);
        } else if let JsObjectInternalObject::ComponentMut(_) = self.internal_object {
            object_template.set_internal_field_count(1);
        }

        let object = object_template.new_instance(scope).unwrap();
        let key = v8::String::new(scope, name).unwrap();

        // If we reference a rust object, add the intern object reference to the js object
        // This will be used to access this on methods
        if let JsObjectInternalObject::Service(internal_object) = &mut self.internal_object {
            let ref_value = v8::External::new(
                scope,
                internal_object as *mut Arc<RwLock<Box<dyn Service>>> as *mut c_void,
            );

            object.set_internal_field(0, ref_value.into());
        } else if let JsObjectInternalObject::Component(internal_object) = &self.internal_object {
            let ref_value =
                v8::External::new(scope, internal_object as *const ComponentRef as *mut c_void);

            object.set_internal_field(0, ref_value.into());
        } else if let JsObjectInternalObject::ComponentMut(internal_object) = &self.internal_object
        {
            let ref_value =
                v8::External::new(scope, internal_object as *const ComponentMut as *mut c_void);

            object.set_internal_field(0, ref_value.into());
        }

        // Add the fieds
        self.fields
            .iter_mut()
            .for_each(|(name, field)| field.register(scope, name, object));

        parent.set(scope, key.into(), object.into());
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}

fn service_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as service
    let this = args.this().get_internal_field(scope, 0).unwrap();
    let this = unsafe { v8::Local::<v8::External>::cast(this) };
    let this = this.value() as *const Arc<RwLock<Box<dyn Service>>>;
    let this = unsafe { this.as_ref().unwrap().clone() };

    // Extract the current method info
    let method_info = {
        let reader = this.read().unwrap();
        let this = reader.deref();

        let method_infos = this.get_method_infos();
        let name = args
            .data()
            .unwrap()
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        method_infos
            .iter()
            .find(|method_info| method_info.name == name)
            .unwrap()
            .clone()
    };

    // Build the arguments
    let deserialized_args = (0..args.length())
        .filter_map(|index| deserialize_v8(scope, args.get(index)))
        .collect::<Vec<_>>();

    // Call the function
    let result = match method_info.call {
        MethodCaller::Const(call) => {
            let reader = this.read().unwrap();
            let this = &**reader.deref();
            call(this.as_any_ref(), deserialized_args).unwrap()
        }
        MethodCaller::Mut(call) => {
            let mut writer = this.write().unwrap();
            let this = &mut **writer.deref_mut();
            call(this.as_any_mut(), deserialized_args).unwrap()
        }
    };

    // Return the result
    if let Some(result) = result {
        let serialized = serialize_v8(scope, result);

        if let Some(serialized) = serialized {
            return_value.set(serialized.into());
        }
    }
}
