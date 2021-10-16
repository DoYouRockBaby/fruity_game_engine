use fruity_any_derive::*;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_rwlock::ServiceRwLock;
use fruity_ecs::service::utils::cast_next_argument;
use fruity_ecs::service::utils::cast_service;
use fruity_ecs::system::system_manager::SystemManager;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

#[derive(Debug, FruityAny)]
pub struct WindowsManager {
    system_manager: ServiceRwLock<SystemManager>,
    event_stack: Arc<RwLock<Vec<FruityWindowsEvent>>>,
    window: RwLock<Option<Window>>,
}

#[derive(Debug)]
enum FruityWindowsEvent {
    Close,
}

impl WindowsManager {
    pub fn new(system_manager: ServiceRwLock<SystemManager>) -> WindowsManager {
        WindowsManager {
            system_manager,
            event_stack: Arc::new(RwLock::new(Vec::new())),
            window: RwLock::new(None),
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
            let mut window_writer = self.window.write().unwrap();
            *window_writer.deref_mut() = Some(window);
            window_id
        };

        // Run the event loop
        let system_manager = self.system_manager.clone();
        let event_stack = self.event_stack.clone();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // Check if the user has closed the window from the OS
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: event_window_id,
                ..
            } = event
            {
                if event_window_id == window_id {
                    *control_flow = ControlFlow::Exit;
                }
            }

            // Check custom events
            let mut events = event_stack.write().unwrap();
            while let Some(event) = events.pop() {
                match event {
                    FruityWindowsEvent::Close => *control_flow = ControlFlow::Exit,
                }
            }

            // Run the systems
            let system_manager_writer = system_manager.write().unwrap();
            system_manager_writer.run();
        });
    }

    pub fn close(&self) {
        let mut events = self.event_stack.write().unwrap();
        events.push(FruityWindowsEvent::Close);
    }

    pub fn set_resizable(&self, resizable: bool) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            window.set_resizable(resizable);
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
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
            window.set_inner_size(LogicalSize::new(width as i32, height as i32));
        }
    }

    pub fn set_title(&self, title: &str) {
        let window = self.window.read().unwrap();
        if let Some(window) = window.as_ref() {
            window.set_title(title);
        }
    }
}

impl IntrospectMethods<Serialized> for WindowsManager {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        vec![
            MethodInfo {
                name: "run".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    this.run();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "close".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_service::<WindowsManager>(this);
                    this.close();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_resizable".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, mut args| {
                    let this = cast_service::<WindowsManager>(this);

                    let arg1 = cast_next_argument("set_resizable", &mut args, |arg| arg.as_bool())?;

                    this.set_resizable(arg1);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "get_size".to_string(),
                args: vec![],
                return_type: None,
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
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, mut args| {
                    let this = cast_service::<WindowsManager>(this);

                    let arg1 = cast_next_argument("set_size", &mut args, |arg| arg.as_usize())?;
                    let arg2 = cast_next_argument("set_size", &mut args, |arg| arg.as_usize())?;

                    this.set_size(arg1, arg2);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_title".to_string(),
                args: vec![],
                return_type: None,
                call: MethodCaller::Const(Arc::new(|this, mut args| {
                    let this = cast_service::<WindowsManager>(this);

                    let arg1 = cast_next_argument("set_title", &mut args, |arg| arg.as_string())?;

                    this.set_title(&arg1);
                    Ok(None)
                })),
            },
        ]
    }
}

impl Service for WindowsManager {}
