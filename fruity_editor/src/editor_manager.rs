use crate::Panes;
use crate::World;
use core::ffi::c_void;
use fruity_any::*;
use fruity_core::service::service::Service;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_windows::windows_manager::FruityWindowsEvent;
use fruity_windows::windows_manager::WindowsManager;
use iced::futures::task::SpawnExt;
use iced_wgpu::wgpu;
use iced_wgpu::Backend;
use iced_wgpu::Renderer;
use iced_wgpu::Settings;
use iced_winit::conversion;
use iced_winit::futures::executor::LocalPool;
use iced_winit::program;
use iced_winit::program::State;
use iced_winit::winit::dpi::PhysicalPosition;
use iced_winit::winit::event::Event;
use iced_winit::winit::event::ModifiersState;
use iced_winit::winit::event::WindowEvent;
use iced_winit::winit::window::Window;
use iced_winit::Clipboard;
use iced_winit::Debug as IcedDebug;
use iced_winit::Size;
use iced_winit::Viewport;
use std::fmt::Debug;
use std::ops::Deref;

pub struct EditorManagerState {
    debug: IcedDebug,
    renderer: Renderer,
    state: State<Panes>,
    viewport: Viewport,
    staging_belt: wgpu::util::StagingBelt,
    modifiers: ModifiersState,
    clipboard: Clipboard,
    cursor_position: PhysicalPosition<f64>,
    local_pool: LocalPool,
}

#[derive(FruityAny)]
pub struct EditorManager {
    windows_manager: ServiceRwLock<WindowsManager>,
    graphic_manager: ServiceRwLock<GraphicsManager>,
    state: Option<EditorManagerState>,
    panes: Option<Panes>,
}

unsafe impl Send for EditorManager {}
unsafe impl Sync for EditorManager {}

impl Debug for EditorManager {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl EditorManager {
    pub fn new(world: &World) -> EditorManager {
        let service_manager_reader = world.service_manager.read().unwrap();
        let windows_manager = service_manager_reader.get::<WindowsManager>().unwrap();
        let graphic_manager = service_manager_reader.get::<GraphicsManager>().unwrap();
        let windows_manager_reader = windows_manager.read().unwrap();
        let graphic_manager_reader = graphic_manager.read().unwrap();

        // Register to events of graphic_manager to update the UI when needed
        let service_manager = world.service_manager.clone();
        graphic_manager_reader
            .on_initialized
            .add_observer(move |_| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.initialize();
            });

        let service_manager = world.service_manager.clone();
        graphic_manager_reader
            .on_before_draw_end
            .add_observer(move |_| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.draw();
            });

        let service_manager = world.service_manager.clone();
        graphic_manager_reader
            .on_after_draw_end
            .add_observer(move |_| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.call_staging_buffer();
            });

        let service_manager = world.service_manager.clone();
        windows_manager_reader.on_event.add_observer(move |event| {
            let service_manager = service_manager.read().unwrap();
            let mut editor_manager = service_manager.write::<EditorManager>();

            // TODO: Wait the release of the next version of iced to remove it
            let event = unsafe {
                std::mem::transmute::<
                    winit::event::Event<FruityWindowsEvent>,
                    Event<FruityWindowsEvent>,
                >(event.clone())
            };

            editor_manager.handle_event(&event);
        });

        let service_manager = world.service_manager.clone();
        windows_manager_reader
            .on_events_cleared
            .add_observer(move |_| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.process_event_queue();
            });

        let service_manager = world.service_manager.clone();
        windows_manager_reader
            .on_resize
            .add_observer(move |(width, height)| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.handle_viewport_resize(width, height);
            });

        let service_manager = world.service_manager.clone();
        windows_manager_reader
            .on_cursor_moved
            .add_observer(move |(x, y)| {
                let service_manager = service_manager.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.handle_cursor_move(x, y);
            });

        // Create the base UI
        let panes = Panes::new(world);

        EditorManager {
            windows_manager: windows_manager.clone(),
            graphic_manager: graphic_manager.clone(),
            panes: Some(panes),
            state: None,
        }
    }

    pub fn initialize(&mut self) {
        // Get all what we need to initialize
        let windows_manager = self.windows_manager.read().unwrap();
        let graphic_manager = self.graphic_manager.read().unwrap();
        let device = graphic_manager.get_device().unwrap();
        let config = graphic_manager.get_config().unwrap();

        // Initialize the renderer
        let mut debug = IcedDebug::new();
        let mut renderer = Renderer::new(Backend::new(device, Settings::default(), config.format));
        let size = windows_manager.get_size();
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let clipboard = {
            let window = windows_manager.get_window().unwrap();
            let window = window.read().unwrap();

            // TODO: Wait the release of the next version of iced to remove it
            let window = window.deref() as *const _ as *const c_void;
            let window = window as *const Window;
            let window = unsafe { &*window as &Window };
            Clipboard::connect(window)
        };

        // Initialize staging belt and local pool
        let staging_belt = wgpu::util::StagingBelt::new(5 * 1024);
        let local_pool = LocalPool::new();

        // Initialize viewport
        let viewport = Viewport::with_physical_size(
            Size::new(size.0 as u32, size.1 as u32),
            windows_manager.get_scale_factor(),
        );

        // Create the UI State that will be used to manage the UI
        let modifiers = ModifiersState::default();
        let state = program::State::new(
            self.panes.take().unwrap(),
            viewport.logical_size(),
            conversion::cursor_position(cursor_position, windows_manager.get_scale_factor()),
            &mut renderer,
            &mut debug,
        );

        self.state = Some(EditorManagerState {
            debug,
            renderer,
            state,
            viewport,
            staging_belt,
            modifiers,
            clipboard,
            cursor_position,
            local_pool,
        });
    }

    fn draw(&mut self) {
        // Get all what we need to draw
        let graphic_manager = self.graphic_manager.read().unwrap();
        let device = graphic_manager.get_device().unwrap();
        let rendering_view = graphic_manager.get_rendering_view().unwrap();
        let mut encoder = graphic_manager.get_encoder().unwrap().write().unwrap();

        if let Some(state) = &mut self.state {
            let mouse_interaction = state.renderer.backend_mut().draw(
                &device,
                &mut state.staging_belt,
                &mut encoder,
                &rendering_view,
                &state.viewport,
                state.state.primitive(),
                &state.debug.overlay(),
            );

            state.staging_belt.finish();

            // Update the cursor
            let windows_manager = self.windows_manager.read().unwrap();
            let window = windows_manager.get_window().unwrap();
            let window = window.read().unwrap();

            // TODO: Wait the release of the next version of iced to remove it
            let window = window.deref() as *const _ as *const c_void;
            let window = window as *const Window;
            let window = unsafe { &*window as &Window };
            window.set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));
        }
    }

    fn handle_cursor_move(&mut self, x: &usize, y: &usize) {
        if let Some(state) = &mut self.state {
            state.cursor_position = PhysicalPosition::new(*x as f64, *y as f64);
        }
    }

    fn handle_viewport_resize(&mut self, width: &usize, height: &usize) {
        if let Some(state) = &mut self.state {
            // and request a redraw
            let windows_manager = self.windows_manager.read().unwrap();
            let window = windows_manager.get_window().unwrap();
            let window = window.read().unwrap();

            // TODO: Wait the release of the next version of iced to remove it
            let window = window.deref() as *const _ as *const c_void;
            let window = window as *const Window;
            let window = unsafe { &*window as &Window };
            window.request_redraw();

            state.viewport = Viewport::with_physical_size(
                Size::new(*width as u32, *height as u32),
                window.scale_factor(),
            );
        }
    }

    fn handle_event(&mut self, event: &Event<'static, FruityWindowsEvent>) {
        if let Some(state) = &mut self.state {
            let windows_manager = self.windows_manager.read().unwrap();

            if let Event::WindowEvent { event, .. } = &event {
                if let WindowEvent::ModifiersChanged(new_modifiers) = event {
                    state.modifiers = *new_modifiers;
                } else if let Some(event) = iced_winit::conversion::window_event(
                    &event,
                    windows_manager.get_scale_factor(),
                    state.modifiers,
                ) {
                    state.state.queue_event(event);
                }
            }
        }
    }

    fn process_event_queue(&mut self) {
        if let Some(state) = &mut self.state {
            let windows_manager = self.windows_manager.read().unwrap();

            // If there are events pending
            if !state.state.is_queue_empty() {
                // We update iced
                state.state.update(
                    state.viewport.logical_size(),
                    conversion::cursor_position(
                        state.cursor_position,
                        windows_manager.get_scale_factor(),
                    ),
                    &mut state.renderer,
                    &mut state.clipboard,
                    &mut state.debug,
                );

                // and request a redraw
                let window = windows_manager.get_window().unwrap();
                let window = window.read().unwrap();

                // TODO: Wait the release of the next version of iced to remove it
                let window = window.deref() as *const _ as *const c_void;
                let window = window as *const Window;
                let window = unsafe { &*window as &Window };
                window.request_redraw();
            }
        }
    }

    fn call_staging_buffer(&mut self) {
        if let Some(state) = &mut self.state {
            state
                .local_pool
                .spawner()
                .spawn(state.staging_belt.recall())
                .expect("Recall staging buffers");

            state.local_pool.run_until_stalled();
        }
    }
}

impl IntrospectObject for EditorManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for EditorManager {}
