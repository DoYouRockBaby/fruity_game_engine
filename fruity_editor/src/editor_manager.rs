use crate::hooks::use_global;
use crate::state::entity::EntityState;
use crate::ui_element::egui::app::Application;
use egui::FontDefinitions;
use egui_wgpu_backend::RenderPass;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::Platform;
use egui_winit_platform::PlatformDescriptor;
use epi::*;
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
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;
use winit::event::Event;

pub struct EditorManagerState {
    platform: Platform,
    egui_rpass: RenderPass,
    start_time: Instant,
    previous_frame_time: Option<f32>,
    application: Application,
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
        windows_manager_reader.on_event.add_observer(move |event| {
            let service_manager = service_manager_2.read().unwrap();
            let mut editor_manager = service_manager.write::<EditorManager>();

            editor_manager.handle_event(&event);
        });

        // Create the base UI
        let application = Application::new(service_manager);

        // Connect to the window
        let state = EditorManager::initialize(
            application,
            windows_manager.clone(),
            &graphic_manager_reader,
        );

        EditorManager {
            windows_manager: windows_manager.clone(),
            graphic_manager: graphic_manager.clone(),
            state,
        }
    }

    pub fn initialize(
        application: Application,
        windows_manager: ServiceRwLock<WindowsManager>,
        graphic_manager: &GraphicsManager,
    ) -> EditorManagerState {
        let windows_manager_reader = windows_manager.read().unwrap();

        // Get all what we need to initialize
        let device = graphic_manager.get_device();
        let config = graphic_manager.get_config();

        // We use the egui_winit_platform crate as the platform.
        let size = windows_manager_reader.get_size();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.0 as u32,
            physical_height: size.1 as u32,
            scale_factor: windows_manager_reader.get_scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&device, config.format, 1);

        EditorManagerState {
            platform,
            egui_rpass,
            start_time: Instant::now(),
            previous_frame_time: None,
            application,
        }
    }

    fn draw(&mut self) {
        let graphic_manager = self.graphic_manager.read().unwrap();
        let windows_manager = self.windows_manager.read().unwrap();
        let device = graphic_manager.get_device();
        let config = graphic_manager.get_config();
        let queue = graphic_manager.get_queue();
        let rendering_view = graphic_manager.get_rendering_view();
        let encoder = graphic_manager.get_encoder().unwrap();

        self.state
            .platform
            .update_time(self.state.start_time.elapsed().as_secs_f64());

        // Begin to draw the UI frame.
        let egui_start = Instant::now();
        self.state.platform.begin_frame();

        // Draw the demo application
        self.state.application.draw(&self.state.platform);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self
            .state
            .platform
            .end_frame(Some(windows_manager.get_window()));
        let paint_jobs = self.state.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.state.previous_frame_time = Some(frame_time);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: config.width,
            physical_height: config.height,
            scale_factor: windows_manager.get_scale_factor() as f32,
        };
        self.state.egui_rpass.update_texture(
            &device,
            &queue,
            &self.state.platform.context().texture(),
        );
        self.state.egui_rpass.update_user_textures(&device, &queue);
        self.state
            .egui_rpass
            .update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);

        // Record all render passes.
        let mut encoder = encoder.write().unwrap();
        self.state
            .egui_rpass
            .execute(
                &mut encoder,
                &rendering_view,
                &paint_jobs,
                &screen_descriptor,
                None,
            )
            .unwrap();
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

    fn handle_event(&mut self, event: &Event<'static, ()>) {
        self.state.platform.handle_event(&event);
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
