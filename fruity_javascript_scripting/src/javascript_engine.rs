use crate::configure_components;
use crate::configure_services;
use crate::error::log_js_error;
use crate::JsRuntime;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::world::World;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

pub(crate) enum RuntimeEvent {
    RunScript {
        source: String,
        notify_done_sender: mpsc::Sender<()>,
    },
    RunModule {
        path: String,
        notify_done_sender: mpsc::Sender<()>,
    },
    RunCallback {
        identifier: CallbackIdentifier,
        args: Vec<Serialized>,
        notify_done_sender: mpsc::Sender<()>,
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
        let (loading_sender, loading_receiver) = mpsc::channel::<()>();

        // Create a thread that will be dedicated to the javascript runtime
        // An event channel will be used to make the runtime do what we want
        let service_manager = world.service_manager.clone();
        thread::spawn(move || {
            let mut runtime = JsRuntime::new();
            configure_services(&mut runtime, service_manager.clone());
            configure_components(&mut runtime, service_manager.clone());

            loading_sender.send(()).unwrap();

            for received in receiver {
                match received {
                    RuntimeEvent::RunScript {
                        source,
                        notify_done_sender,
                    } => {
                        match runtime.run_script(&source) {
                            Ok(_) => (),
                            Err(err) => log_js_error(&err),
                        };

                        notify_done_sender.send(()).unwrap();
                    }
                    RuntimeEvent::RunModule {
                        path,
                        notify_done_sender,
                    } => {
                        match runtime.run_module(&path) {
                            Ok(_) => (),
                            Err(err) => log_js_error(&err),
                        };

                        notify_done_sender.send(()).unwrap();
                    }
                    RuntimeEvent::RunCallback {
                        identifier,
                        args,
                        notify_done_sender,
                    } => {
                        runtime.run_stored_callback(identifier, args);
                        notify_done_sender.send(()).unwrap();
                    }
                };
            }
        });

        loading_receiver.recv().unwrap();

        JavascriptEngine {
            channel_sender: sender,
        }
    }

    pub fn run_script(&self, source: &str) {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<()>();

        match self.channel_sender.send(RuntimeEvent::RunScript {
            source: source.to_string(),
            notify_done_sender,
        }) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        };

        notify_done_receiver.recv().unwrap();
    }

    pub fn run_module(&self, path: &str) {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<()>();

        match self.channel_sender.send(RuntimeEvent::RunModule {
            path: path.to_string(),
            notify_done_sender,
        }) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        };

        notify_done_receiver.recv().unwrap();
    }

    pub fn run_callback(&self, identifier: CallbackIdentifier, args: Vec<Serialized>) {
        let (notify_done_sender, notify_done_receiver) = mpsc::channel::<()>();

        match self.channel_sender.send(RuntimeEvent::RunCallback {
            identifier,
            args,
            notify_done_sender,
        }) {
            Ok(()) => (),
            Err(err) => log::error!("{}", err.to_string()),
        };

        notify_done_receiver.recv().unwrap();
    }
}

impl Drop for JavascriptEngine {
    fn drop(&mut self) {}
}

impl IntrospectObject for JavascriptEngine {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for JavascriptEngine {}
