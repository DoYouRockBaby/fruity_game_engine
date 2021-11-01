use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::utils::cast_service;
use fruity_core::service::utils::ArgumentCaster;
use fruity_core::signal::Signal;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::fmt::Debug;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::window::Window;

#[derive(FruityAny)]
pub struct WindowsManager {
    window: Window,
    // event_loop_proxy: EventLoopProxy<()>,
    pub on_start_update: Signal<()>,
    pub on_end_update: Signal<()>,
    pub on_resize: Signal<(usize, usize)>,
    pub on_cursor_moved: Signal<(usize, usize)>,
    pub on_event: Signal<Event<'static, ()>>,
    pub on_events_cleared: Signal<()>,
}

impl Debug for WindowsManager {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl WindowsManager {
    pub fn new(window: Window) -> WindowsManager {
        WindowsManager {
            window,
            on_start_update: Signal::new(),
            on_end_update: Signal::new(),
            on_resize: Signal::new(),
            on_cursor_moved: Signal::new(),
            on_event: Signal::new(),
            on_events_cleared: Signal::new(),
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn close(&self) {
        // TODO: Repair that
        //window.se.push(WindowEvent::CloseRequested);
    }

    pub fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    pub fn get_size(&self) -> (usize, usize) {
        (
            self.window.inner_size().width as usize,
            self.window.inner_size().height as usize,
        )
    }

    pub fn get_scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn set_size(&self, width: usize, height: usize) {
        self.window
            .set_inner_size(LogicalSize::new(width as i32, height as i32));
        self.on_resize.notify((width, height))
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}

impl IntrospectObject for WindowsManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
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

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for WindowsManager {}
