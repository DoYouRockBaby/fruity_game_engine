use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::utils::single_thread_wrapper::SingleThreadWrapper;
use fruity_ecs::system::system_service::SystemService;
use fruity_javascript::javascript_service::JavascriptService;
use fruity_windows::window_service::WindowService;
use notify::op::*;
use notify::raw_watcher;
use notify::RawEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::fmt::Debug;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct CallbackIdentifier(pub i32);

pub struct InnerJavascriptWatcherService {
    module_path: String,
    watch_event_receiver: Receiver<RawEvent>,
    _watcher: RecommendedWatcher,
}

impl Debug for InnerJavascriptWatcherService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, FruityAny)]
pub struct JavascriptWatcherService {
    javascript_service: ResourceReference<JavascriptService>,
    system_service: ResourceReference<SystemService>,
    single_thread_wrappers: Vec<SingleThreadWrapper<InnerJavascriptWatcherService>>,
}

impl JavascriptWatcherService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> Self {
        let javascript_service = resource_container.require::<JavascriptService>();
        let system_service = resource_container.require::<SystemService>();
        let window_service = resource_container.require::<dyn WindowService>();
        let window_service = window_service.read();

        // Subscribe to windows observer to proceed the hot reload before the system run
        let resource_container_2 = resource_container.clone();
        window_service.on_start_update().add_observer(move |_| {
            let watcher_service = resource_container_2.require::<JavascriptWatcherService>();
            let watcher_service = watcher_service.read();

            watcher_service.update_watch();
        });

        Self {
            javascript_service,
            system_service,
            single_thread_wrappers: Vec::new(),
        }
    }

    pub fn watch_module(&mut self, path: &str, folder: &str) {
        let module_path = path.to_string();
        let folder = folder.to_string();

        let javascript_service = self.javascript_service.read();
        javascript_service.run_module(&module_path);

        let single_thread_wrapper =
            SingleThreadWrapper::<InnerJavascriptWatcherService>::start(move || {
                let (tx, watch_event_receiver) = channel();

                let mut watcher = raw_watcher(tx).unwrap();
                watcher.watch(&folder, RecursiveMode::NonRecursive).unwrap();

                InnerJavascriptWatcherService {
                    module_path,
                    watch_event_receiver,
                    _watcher: watcher,
                }
            });

        self.single_thread_wrappers.push(single_thread_wrapper);
    }

    pub fn update_watch(&self) {
        self.single_thread_wrappers
            .iter()
            .for_each(|single_thread_wrapper| {
                let system_service = self.system_service.clone();
                let javascript_service = self.javascript_service.clone();

                single_thread_wrapper.call(move |inner| {
                    let system_service = system_service.clone();
                    let javascript_service = javascript_service.clone();

                    inner.watch_event_receiver.try_iter().for_each(|event| {
                        if let RawEvent {
                            path: Some(path),
                            op: Ok(op),
                            cookie: _,
                        } = event
                        {
                            if let Some(extension) = path.as_path().extension() {
                                if extension.to_string_lossy() == "js" {
                                    if op == CREATE || op == REMOVE || op == WRITE {
                                        // Unload all system of the script
                                        {
                                            let mut system_service = system_service.write();
                                            system_service.unload_origin(&inner.module_path);
                                        }

                                        // Reload the module
                                        let javascript_service = javascript_service.read();
                                        javascript_service.reset();
                                        javascript_service.run_module(&inner.module_path);
                                    }
                                }
                            }
                        }
                    });
                });
            });
    }
}

impl IntrospectObject for JavascriptWatcherService {
    fn get_class_name(&self) -> String {
        "JavascriptWatcherService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for JavascriptWatcherService {}
