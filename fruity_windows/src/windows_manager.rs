use fruity_any::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::service::utils::cast_service;
use fruity_ecs::service::utils::ArgumentCaster;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use fruity_observer::Signal;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

#[derive(FruityAnySyncSend)]
pub struct WindowsManager {
    system_manager: ServiceRwLock<SystemManager>,
    event_stack: Arc<RwLock<Vec<FruityWindowsEvent>>>,
    window: RwLock<Option<Arc<RwLock<Window>>>>,
    pub on_windows_creation: Signal<()>,
    pub on_starting_event_loop: Signal<()>,
    pub on_start_update: Signal<()>,
    pub on_end_update: Signal<()>,
    pub on_resize: Signal<(usize, usize)>,
}

#[derive(Debug)]
pub enum FruityWindowsEvent {
    Close,
}

impl Debug for WindowsManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl WindowsManager {
    pub fn new(system_manager: ServiceRwLock<SystemManager>) -> WindowsManager {
        WindowsManager {
            system_manager,
            event_stack: Arc::new(RwLock::new(Vec::new())),
            window: RwLock::new(None),
            on_windows_creation: Signal::new(),
            on_starting_event_loop: Signal::new(),
            on_start_update: Signal::new(),
            on_end_update: Signal::new(),
            on_resize: Signal::new(),
        }
    }

    pub fn run(&self) {
        // Build the window
        let event_loop = EventLoop::<FruityWindowsEvent>::with_user_event();
        let window_id = {
            let window = WindowBuilder::new()
                .with_title("Hit space to toggle resizability.")
                .with_inner_size(LogicalSize::new(400, 200))
                .with_resizable(true)
                .build(&event_loop)
                .unwrap();

            let window_id = window.id();
            let window = Arc::new(RwLock::new(window));

            let mut window_writer = self.window.write().unwrap();
            *window_writer = Some(window);

            window_id
        };
        self.on_windows_creation.notify(());

        // Run the event loop
        let system_manager = self.system_manager.clone();
        let event_stack = self.event_stack.clone();
        self.on_starting_event_loop.notify(());

        let on_start_update = self.on_start_update.clone();
        let on_end_update = self.on_end_update.clone();
        let on_resize = self.on_resize.clone();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                // Check if the user has closed the window from the OS
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id: event_window_id,
                    ..
                } => {
                    if event_window_id == window_id {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                // Check if the user has resized the window from the OS
                Event::WindowEvent {
                    event: WindowEvent::Resized(physical_size),
                    ..
                } => {
                    on_resize.notify((physical_size.width as usize, physical_size.height as usize));
                }
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
                    ..
                } => {
                    on_resize.notify((
                        new_inner_size.width as usize,
                        new_inner_size.height as usize,
                    ));
                }
                _ => (),
            }

            // Check custom events
            let mut events = event_stack.write().unwrap();
            while let Some(event) = events.pop() {
                match event {
                    FruityWindowsEvent::Close => *control_flow = ControlFlow::Exit,
                }
            }

            // Start updating
            on_start_update.notify(());

            // Run the systems
            let system_manager_writer = system_manager.write().unwrap();
            system_manager_writer.run();

            // End the update
            on_end_update.notify(());
        });
    }

    pub fn get_window(&self) -> Option<Arc<RwLock<Window>>> {
        let window = self.window.read().ok()?;
        window.as_ref().map(|window| window.clone())
    }

    pub fn close(&self) {
        let mut events = self.event_stack.write().unwrap();
        events.push(FruityWindowsEvent::Close);
    }

    pub fn set_resizable(&self, resizable: bool) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            let window = window.read().unwrap();
            window.set_resizable(resizable);
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            let window = window.read().unwrap();
            (
                window.inner_size().width as usize,
                window.inner_size().height as usize,
            )
        } else {
            (0, 0)
        }
    }

    pub fn set_size(&self, width: usize, height: usize) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            let window = window.read().unwrap();
            window.set_inner_size(LogicalSize::new(width as i32, height as i32));
            self.on_resize.notify((width, height))
        }
    }

    pub fn set_title(&self, title: &str) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            let window = window.read().unwrap();
            window.set_title(title);
        }
    }
}

impl IntrospectMethods<Serialized> for WindowsManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
                name: "run".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    this.run();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "close".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    this.close();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_resizable".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<WindowsManager>(this);

                    let mut caster = ArgumentCaster::new("set_resizable", args);
                    let arg1 = caster.cast_next::<bool>()?;

                    this.set_resizable(arg1);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "get_size".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    let result = this.get_size();

                    Ok(Some(Serialized::Array(vec![
                        Serialized::USize(result.0),
                        Serialized::USize(result.1),
                    ])))
                })),
            },
            MethodInfo {
                name: "set_size".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<WindowsManager>(this);

                    let mut caster = ArgumentCaster::new("set_size", args);
                    let arg1 = caster.cast_next::<usize>()?;
                    let arg2 = caster.cast_next::<usize>()?;

                    this.set_size(arg1, arg2);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_title".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_service::<WindowsManager>(this);

                    let mut caster = ArgumentCaster::new("set_title", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.set_title(&arg1);
                    Ok(None)
                })),
            },
        ]
    }
}

impl Service for WindowsManager {}
