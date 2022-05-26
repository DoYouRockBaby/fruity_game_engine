use crate::ui_element::app::Application;
use crate::ui_element::app::DrawContext;
use egui::FontDefinitions;
use egui_wgpu_backend::RenderPass;
use egui_wgpu_backend::ScreenDescriptor;
use egui_winit_platform::Platform;
use egui_winit_platform::PlatformDescriptor;
use epi::*;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_graphic::graphic_service::GraphicService;
use fruity_wgpu_graphic::graphic_service::WgpuGraphicService;
use fruity_windows::window_service::WindowService;
use fruity_winit_windows::window_service::WinitWindowService;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Instant;
use winit::event::Event;

pub struct EditorServiceState {
    platform: Platform,
    egui_rpass: RenderPass,
    start_time: Instant,
    previous_frame_time: Option<f32>,
    application: Application,
}

#[derive(FruityAny)]
pub struct EditorService {
    window_service: ResourceReference<dyn WindowService>,
    graphic_service: ResourceReference<dyn GraphicService>,
    state: EditorServiceState,
}

impl Debug for EditorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl EditorService {
    pub fn new(resource_container: Arc<ResourceContainer>) -> EditorService {
        let window_service = resource_container.require::<dyn WindowService>();
        let graphic_service = resource_container.require::<dyn GraphicService>();

        let window_service_reader = window_service.read();
        let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();
        let graphic_service_reader = graphic_service.read();

        // Register to events of graphic_service to update the UI when needed
        let resource_container_2 = resource_container.clone();
        graphic_service_reader
            .on_before_draw_end()
            .add_observer(move |_| {
                puffin::profile_scope!("draw_editor");

                let editor_service = resource_container_2.require::<EditorService>();
                let mut editor_service = editor_service.write();

                editor_service.draw();
            });

        let resource_container_2 = resource_container.clone();
        window_service_reader.on_event.add_observer(move |event| {
            let editor_service = resource_container_2.require::<EditorService>();
            let mut editor_service = editor_service.write();

            editor_service.handle_event(&event);
        });

        // Create the base UI
        let application = Application::new(resource_container.clone());

        // Connect to the window
        let state =
            EditorService::initialize(application, window_service.clone(), graphic_service.clone());

        EditorService {
            window_service: window_service.clone(),
            graphic_service: graphic_service.clone(),
            state,
        }
    }

    pub fn initialize(
        application: Application,
        window_service: ResourceReference<dyn WindowService>,
        graphic_service: ResourceReference<dyn GraphicService>,
    ) -> EditorServiceState {
        let window_service = window_service.read();
        let graphic_service = graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

        // Get all what we need to initialize
        let device = graphic_service.get_device();
        let config = graphic_service.get_config();

        // We use the egui_winit_platform crate as the platform.
        let size = window_service.get_windows_size();
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.0 as u32,
            physical_height: size.1 as u32,
            scale_factor: window_service.get_scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let egui_rpass = RenderPass::new(&device, config.format, 1);

        EditorServiceState {
            platform,
            egui_rpass,
            start_time: Instant::now(),
            previous_frame_time: None,
            application,
        }
    }

    fn draw(&mut self) {
        let window_service = self.window_service.read();
        let window_service = window_service.downcast_ref::<WinitWindowService>();
        let graphic_service = self.graphic_service.read();
        let graphic_service = graphic_service.downcast_ref::<WgpuGraphicService>();

        let device = graphic_service.get_device();
        let config = graphic_service.get_config();
        let queue = graphic_service.get_queue();
        let rendering_view = graphic_service.get_rendering_view();
        let encoder = graphic_service.get_encoder().unwrap();

        self.state
            .platform
            .update_time(self.state.start_time.elapsed().as_secs_f64());

        // Begin to draw the UI frame.
        let egui_start = Instant::now();
        self.state.platform.begin_frame();

        // Draw the application
        self.state.application.draw(&mut DrawContext {
            device: device,
            platform: &self.state.platform,
            egui_rpass: &mut self.state.egui_rpass,
        });

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self
            .state
            .platform
            .end_frame(Some(window_service.get_window()));
        let paint_jobs = self.state.platform.context().tessellate(paint_commands);

        let frame_time = (Instant::now() - egui_start).as_secs_f64() as f32;
        self.state.previous_frame_time = Some(frame_time);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: config.width,
            physical_height: config.height,
            scale_factor: window_service.get_scale_factor() as f32,
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

    fn handle_event(&mut self, event: &Event<'static, ()>) {
        self.state.platform.handle_event(&event);
    }

    pub fn get_egui_rpass(&mut self) -> &mut RenderPass {
        &mut self.state.egui_rpass
    }
}

impl IntrospectObject for EditorService {
    fn get_class_name(&self) -> String {
        "EditorService".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for EditorService {}
