use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::ui_element::iced::program::Program;
use core::ffi::c_void;
use fruity_any::*;
use fruity_core::entity::entity::EntityId;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_core::service::utils::cast_service;
use fruity_core::service::utils::ArgumentCaster;
use fruity_graphic::graphics_manager::GraphicsManager;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
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
use std::sync::Arc;
use std::sync::RwLock;

pub struct EditorManagerState {
    debug: IcedDebug,
    renderer: Renderer,
    state: State<Program>,
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
    state: EditorManagerState,
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
    pub fn new(service_manager: &Arc<RwLock<ServiceManager>>) -> EditorManager {
        let service_manager_reader = service_manager.read().unwrap();
        let windows_manager = service_manager_reader.get::<WindowsManager>().unwrap();
        let graphic_manager = service_manager_reader.get::<GraphicsManager>().unwrap();
        let windows_manager_reader = windows_manager.read().unwrap();
        let graphic_manager_reader = graphic_manager.read().unwrap();

        // Register to events of graphic_manager to update the UI when needed
        let service_manager_2 = service_manager.clone();
        graphic_manager_reader
            .on_before_draw_end
            .add_observer(move |_| {
                let service_manager = service_manager_2.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.draw();
            });

        let service_manager_2 = service_manager.clone();
        graphic_manager_reader
            .on_after_draw_end
            .add_observer(move |_| {
                let service_manager = service_manager_2.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.call_staging_buffer();
            });

        let service_manager_2 = service_manager.clone();
        windows_manager_reader.on_event.add_observer(move |event| {
            let service_manager = service_manager_2.read().unwrap();
            let mut editor_manager = service_manager.write::<EditorManager>();

            // TODO: Wait the release of the next version of iced to remove it
            let event =
                unsafe { std::mem::transmute::<winit::event::Event<()>, Event<()>>(event.clone()) };

            editor_manager.handle_event(&event);
        });

        let service_manager_2 = service_manager.clone();
        windows_manager_reader
            .on_events_cleared
            .add_observer(move |_| {
                let service_manager = service_manager_2.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.process_event_queue();
            });

        let service_manager_2 = service_manager.clone();
        windows_manager_reader
            .on_resize
            .add_observer(move |(width, height)| {
                let service_manager = service_manager_2.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.handle_viewport_resize(width, height);
            });

        let service_manager_2 = service_manager.clone();
        windows_manager_reader
            .on_cursor_moved
            .add_observer(move |(x, y)| {
                let service_manager = service_manager_2.read().unwrap();
                let mut editor_manager = service_manager.write::<EditorManager>();

                editor_manager.handle_cursor_move(x, y);
            });

        // Create the base UI
        let panes = Program::new(service_manager);

        // Connect to the window
        let state =
            EditorManager::initialize(panes, &windows_manager_reader, &graphic_manager_reader);

        EditorManager {
            windows_manager: windows_manager.clone(),
            graphic_manager: graphic_manager.clone(),
            state,
        }
    }

    pub fn initialize(
        program: Program,
        windows_manager: &WindowsManager,
        graphic_manager: &GraphicsManager,
    ) -> EditorManagerState {
        // Get all what we need to initialize
        let device = graphic_manager.get_device();
        let config = graphic_manager.get_config();

        // Initialize the renderer
        let mut debug = IcedDebug::new();
        let mut renderer = Renderer::new(Backend::new(device, Settings::default(), config.format));
        let size = windows_manager.get_size();
        let cursor_position = PhysicalPosition::new(-1.0, -1.0);
        let clipboard = {
            let window = windows_manager.get_window();

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
            program,
            viewport.logical_size(),
            conversion::cursor_position(cursor_position, windows_manager.get_scale_factor()),
            &mut renderer,
            &mut debug,
        );

        EditorManagerState {
            debug,
            renderer,
            state,
            viewport,
            staging_belt,
            modifiers,
            clipboard,
            cursor_position,
            local_pool,
        }
    }

    fn draw(&mut self) {
        // Get all what we need to draw
        let graphic_manager = self.graphic_manager.read().unwrap();
        let device = graphic_manager.get_device();
        let rendering_view = graphic_manager.get_rendering_view();
        let mut encoder = graphic_manager.get_encoder().unwrap().write().unwrap();

        let mouse_interaction = self.state.renderer.backend_mut().draw(
            &device,
            &mut self.state.staging_belt,
            &mut encoder,
            &rendering_view,
            &self.state.viewport,
            self.state.state.primitive(),
            &self.state.debug.overlay(),
        );

        self.state.staging_belt.finish();

        // Update the cursor
        let windows_manager = self.windows_manager.read().unwrap();
        let window = windows_manager.get_window();

        // TODO: Wait the release of the next version of iced to remove it
        let window = window.deref() as *const _ as *const c_void;
        let window = window as *const Window;
        let window = unsafe { &*window as &Window };
        window.set_cursor_icon(iced_winit::conversion::mouse_interaction(mouse_interaction));
    }

    fn is_entity_selected(&self, entity_id: &EntityId) -> bool {
        let entity_state = use_global::<EntityState>();
        if let Some(selected_entity) = &entity_state.selected_entity {
            let entity = selected_entity.read();
            entity.entity_id == *entity_id
        } else {
            false
        }
    }

    fn handle_cursor_move(&mut self, x: &usize, y: &usize) {
        self.state.cursor_position = PhysicalPosition::new(*x as f64, *y as f64);
    }

    fn handle_viewport_resize(&mut self, width: &usize, height: &usize) {
        // and request a redraw
        let windows_manager = self.windows_manager.read().unwrap();
        let window = windows_manager.get_window();

        // TODO: Wait the release of the next version of iced to remove it
        let window = window.deref() as *const _ as *const c_void;
        let window = window as *const Window;
        let window = unsafe { &*window as &Window };
        window.request_redraw();

        self.state.viewport = Viewport::with_physical_size(
            Size::new(*width as u32, *height as u32),
            window.scale_factor(),
        );
    }

    fn handle_event(&mut self, event: &Event<'static, ()>) {
        let windows_manager = self.windows_manager.read().unwrap();

        if let Event::WindowEvent { event, .. } = &event {
            if let WindowEvent::ModifiersChanged(new_modifiers) = event {
                self.state.modifiers = *new_modifiers;
            } else if let Some(event) = iced_winit::conversion::window_event(
                &event,
                windows_manager.get_scale_factor(),
                self.state.modifiers,
            ) {
                self.state.state.queue_event(event);
            }
        }
    }

    fn process_event_queue(&mut self) {
        let windows_manager = self.windows_manager.read().unwrap();

        // If there are events pending
        if !self.state.state.is_queue_empty() {
            // We update iced
            self.state.state.update(
                self.state.viewport.logical_size(),
                conversion::cursor_position(
                    self.state.cursor_position,
                    windows_manager.get_scale_factor(),
                ),
                &mut self.state.renderer,
                &mut self.state.clipboard,
                &mut self.state.debug,
            );

            // and request a redraw
            let window = windows_manager.get_window();

            // TODO: Wait the release of the next version of iced to remove it
            let window = window.deref() as *const _ as *const c_void;
            let window = window as *const Window;
            let window = unsafe { &*window as &Window };
            window.request_redraw();
        }
    }

    fn call_staging_buffer(&mut self) {
        self.state
            .local_pool
            .spawner()
            .spawn(self.state.staging_belt.recall())
            .expect("Recall staging buffers");

        self.state.local_pool.run_until_stalled();
    }
}

impl IntrospectObject for EditorManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "is_entity_selected".to_string(),
            call: MethodCaller::Const(Arc::new(|this, args| {
                let this = cast_service::<EditorManager>(this);

                let mut caster = ArgumentCaster::new("is_entity_selected", args);
                let arg1 = caster.cast_next::<EntityId>()?;

                let result = this.is_entity_selected(&arg1);
                Ok(Some(Serialized::Bool(result)))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Service for EditorManager {}
