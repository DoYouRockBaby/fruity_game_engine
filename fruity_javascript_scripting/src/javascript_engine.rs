use crate::configure_components;
use crate::configure_services;
use crate::error::log_js_error;
use crate::JsRuntime;
use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::world::World;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

pub(crate) enum RuntimeEvent {
    RunScript {
        source: String,
    },
    RunModule {
        path: String,
    },
    RunCallback {
        identifier: CallbackIdentifier,
        args: Vec<Serialized>,
    },
}

#[derive(Debug, FruityAny)]
pub struct JavascriptEngine {
    channel_sender: mpsc::SyncSender<RuntimeEvent>,
}

impl JavascriptEngine {
    pub fn new(world: &World) -> JavascriptEngine {
        // TODO: think about a good number for sync channel
        let (sender, receiver) = mpsc::sync_channel::<RuntimeEvent>(10);

        // Create a thread that will be dedicated to the javascript runtime
        // An event channel will be used to make the runtime do what we want
        let service_manager = world.service_manager.clone();
        thread::spawn(move || {
            let mut runtime = JsRuntime::new();
            configure_services(&mut runtime, service_manager.clone());
            configure_components(&mut runtime, service_manager.clone());

            for received in receiver {
                match received {
                    RuntimeEvent::RunScript { source } => {
                        match runtime.run_script(&source) {
                            Ok(_) => (),
                            Err(err) => log_js_error(&err),
                        };
                    }
                    RuntimeEvent::RunModule { path } => {
                        match runtime.run_module(&path) {
                            Ok(_) => (),
                            Err(err) => log_js_error(&err),
                        };
                    }
                    RuntimeEvent::RunCallback { identifier, args } => {
                        runtime.run_stored_callback(identifier, args)
                    }
                };
            }
        });

        JavascriptEngine {
            channel_sender: sender,
        }
    }

    pub fn run_script(&self, source: &str) {
        match self.channel_sender.send(RuntimeEvent::RunScript {
            source: source.to_string(),
        }) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        }
    }

    pub fn run_module(&self, path: &str) {
        match self.channel_sender.send(RuntimeEvent::RunModule {
            path: path.to_string(),
        }) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        }
    }

    pub fn run_callback(&self, identifier: CallbackIdentifier, args: Vec<Serialized>) {
        match self
            .channel_sender
            .send(RuntimeEvent::RunCallback { identifier, args })
        {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        }
    }
}

impl IntrospectMethods<Serialized> for JavascriptEngine {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![]
    }
}

impl Service for JavascriptEngine {}
