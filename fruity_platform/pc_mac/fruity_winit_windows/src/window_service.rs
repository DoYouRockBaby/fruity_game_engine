use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::signal::Signal;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
use fruity_windows::window_service::WindowService;
use std::fmt::Debug;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::window::Window;

#[derive(FruityAny)]
pub struct WinitWindowService {
    window: Window,
    pub cursor_position: (u32, u32),
    pub on_enter_loop: Signal<()>,
    pub on_start_update: Signal<()>,
    pub on_end_update: Signal<()>,
    pub on_resize: Signal<(u32, u32)>,
    pub on_cursor_moved: Signal<(u32, u32)>,
    pub on_event: Signal<Event<'static, ()>>,
    pub on_events_cleared: Signal<()>,
}

impl Debug for WinitWindowService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl WinitWindowService {
    pub fn new(window: Window) -> WinitWindowService {
        WinitWindowService {
            window,
            cursor_position: Default::default(),
            on_enter_loop: Signal::new(),
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
}

impl WindowService for WinitWindowService {
    fn close(&self) {
        // TODO: Repair that
        //window.se.push(WindowEvent::CloseRequested);
    }

    fn set_resizable(&self, resizable: bool) {
        self.window.set_resizable(resizable);
    }

    fn get_windows_size(&self) -> (u32, u32) {
        (
            self.window.inner_size().width,
            self.window.inner_size().height,
        )
    }

    fn get_scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    fn get_cursor_position(&self) -> (u32, u32) {
        self.cursor_position.clone()
    }

    fn set_size(&self, width: u32, height: u32) {
        self.window
            .set_inner_size(LogicalSize::new(width as i32, height as i32));
        self.on_resize.notify((width, height))
    }

    fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    fn on_enter_loop(&self) -> &Signal<()> {
        &self.on_enter_loop
    }

    fn on_start_update(&self) -> &Signal<()> {
        &self.on_start_update
    }

    fn on_end_update(&self) -> &Signal<()> {
        &self.on_end_update
    }

    fn on_resize(&self) -> &Signal<(u32, u32)> {
        &self.on_resize
    }

    fn on_cursor_moved(&self) -> &Signal<(u32, u32)> {
        &self.on_cursor_moved
    }
}

impl IntrospectObject for WinitWindowService {
    fn get_class_name(&self) -> String {
        "WindowService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "close".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);
                    this.close();
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_resizable".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);

                    let mut caster = ArgumentCaster::new("set_resizable", args);
                    let arg1 = caster.cast_next::<bool>()?;

                    this.set_resizable(arg1);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "get_windows_size".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);
                    let result = this.get_windows_size();

                    Ok(Some(Serialized::Array(vec![
                        Serialized::U32(result.0),
                        Serialized::U32(result.1),
                    ])))
                })),
            },
            MethodInfo {
                name: "get_cursor_position".to_string(),
                call: MethodCaller::Const(Arc::new(|this, _args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);
                    let result = this.get_cursor_position();

                    Ok(Some(Serialized::Array(vec![
                        Serialized::U32(result.0),
                        Serialized::U32(result.1),
                    ])))
                })),
            },
            MethodInfo {
                name: "set_size".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);

                    let mut caster = ArgumentCaster::new("set_size", args);
                    let arg1 = caster.cast_next::<u32>()?;
                    let arg2 = caster.cast_next::<u32>()?;

                    this.set_size(arg1, arg2);
                    Ok(None)
                })),
            },
            MethodInfo {
                name: "set_title".to_string(),
                call: MethodCaller::Const(Arc::new(|this, args| {
                    let this = cast_introspect_ref::<WinitWindowService>(this);

                    let mut caster = ArgumentCaster::new("set_title", args);
                    let arg1 = caster.cast_next::<String>()?;

                    this.set_title(&arg1);
                    Ok(None)
                })),
            },
        ]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![
            //TODO: Expose signals
        ]
    }
}

impl Resource for WinitWindowService {}
