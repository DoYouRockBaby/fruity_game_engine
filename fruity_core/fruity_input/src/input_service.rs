use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::Settings;
use fruity_core::signal::Signal;
use fruity_core::utils::introspect::ArgumentCaster;
use fruity_core::utils::introspect::cast_introspect_ref;
use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct InputService {
    pub input_map: HashMap<String, String>,
    pub pressed_inputs: HashSet<String>,
    pub pressed_sources: HashSet<String>,
    pub pressed_this_frame_inputs: HashSet<String>,
    pub pressed_this_frame_sources: HashSet<String>,
    pub released_this_frame_inputs: HashSet<String>,
    pub released_this_frame_sources: HashSet<String>,
    pub on_pressed: Signal<String>,
    pub on_released: Signal<String>,
}

impl Debug for InputService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl InputService {
    pub fn new(_resource_container: Arc<ResourceContainer>) -> InputService {
        InputService {
            input_map: HashMap::new(),
            pressed_inputs: HashSet::new(),
            pressed_sources: HashSet::new(),
            pressed_this_frame_inputs: HashSet::new(),
            pressed_this_frame_sources: HashSet::new(),
            released_this_frame_inputs: HashSet::new(),
            released_this_frame_sources: HashSet::new(),
            on_pressed: Signal::new(),
            on_released: Signal::new(),
        }
    }

    pub fn read_input_settings(&mut self, settings: &Settings) {
        let settings = settings.get_settings("input");
        if let Settings::Object(input_map) = settings {
            input_map.iter().for_each(|(input, sources)| {
                let sources = Vec::<String>::try_from(sources.clone());

                if let Ok(sources) = sources {
                    sources
                        .iter()
                        .for_each(|source| self.register_input(input, source));
                }
            })
        }
    }

    pub fn register_input(&mut self, input: &str, source: &str) {
        self.input_map.insert(source.to_string(), input.to_string());
    }

    pub fn is_pressed(&self, input: &str) -> bool {
        self.pressed_inputs.contains(input)
    }

    pub fn is_source_pressed(&self, source: &str) -> bool {
        self.pressed_sources.contains(source)
    }

    pub fn is_pressed_this_frame(&self, input: &str) -> bool {
        self.pressed_this_frame_inputs.contains(input)
    }

    pub fn is_source_pressed_this_frame(&self, source: &str) -> bool {
        self.pressed_this_frame_sources.contains(source)
    }

    pub fn is_released_this_frame(&self, input: &str) -> bool {
        self.released_this_frame_inputs.contains(input)
    }

    pub fn is_source_released_this_frame(&self, source: &str) -> bool {
        self.released_this_frame_sources.contains(source)
    }

    pub fn notify_pressed(&mut self, source: &str) {
        self.pressed_sources.insert(source.to_string());
        self.pressed_this_frame_sources.insert(source.to_string());

        if let Some(input) = self.input_map.get(source) {
            if !self.pressed_inputs.contains(input) {
                self.pressed_inputs.insert(input.clone());
                self.pressed_this_frame_inputs.insert(input.to_string());
                self.on_pressed.notify(input.clone());
            }
        }
    }

    pub fn notify_released(&mut self, source: &str) {
        self.pressed_sources.remove(source);
        self.released_this_frame_sources.insert(source.to_string());

        if let Some(input) = self.input_map.get(source) {
            if self.pressed_inputs.contains(input) {
                self.pressed_inputs.remove(input);
                self.released_this_frame_inputs.insert(input.to_string());
                self.on_released.notify(input.clone());
            }
        }
    }

    pub fn handle_frame_end(&mut self) {
        self.pressed_this_frame_sources.clear();
        self.pressed_this_frame_inputs.clear();
        self.released_this_frame_sources.clear();
        self.released_this_frame_inputs.clear();
    }
}

impl IntrospectObject for InputService {
    fn get_class_name(&self) -> String {
        "InputService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "is_pressed".to_string(),
            call: MethodCaller::Const(Arc::new(|this, args| {
                let this = cast_introspect_ref::<InputService>(this);

                let mut caster = ArgumentCaster::new("is_pressed", args);
                let arg1 = caster.cast_next::<String>()?;

                let result = this.is_pressed(&arg1);
                Ok(Some(Serialized::Bool(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "on_pressed".to_string(),
                ty: TypeId::of::<Signal<String>>(),
                serializable: false,
                getter: Arc::new(|this| {
                    this.downcast_ref::<InputService>()
                        .unwrap()
                        .on_pressed
                        .clone()
                        .into()
                }),
                setter: SetterCaller::None,
            },
            FieldInfo {
                name: "on_released".to_string(),
                ty: TypeId::of::<Signal<String>>(),
                serializable: false,
                getter: Arc::new(|this| {
                    this.downcast_ref::<InputService>()
                        .unwrap()
                        .on_released
                        .clone()
                        .into()
                }),
                setter: SetterCaller::None,
            },
        ]
    }
}

impl Resource for InputService {}
