use crate::window_service::WinitWindowService;
use core::ffi::c_void;
use fruity_core::platform::Initializer;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use fruity_windows::frame_service::FrameService;
use fruity_windows::window_service::WindowService;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub mod window_service;

struct WindowSettings {
    title: String,
    width: usize,
    height: usize,
    resizable: bool,
}

pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let window_service = resource_container.require::<dyn WindowService>("window_service");
    let window_service = window_service.read();
    let window_service = window_service
        .as_any_ref()
        .downcast_ref::<WinitWindowService>()
        .unwrap();

    let resource_container_2 = resource_container.clone();
    window_service.on_enter_loop.add_observer(move |_| {
        let frame_service = resource_container_2.require::<FrameService>("frame_service");
        let mut frame_service = frame_service.write();

        frame_service.begin_frame();
    });

    let resource_container_2 = resource_container.clone();
    window_service.on_start_update.add_observer(move |_| {
        let frame_service = resource_container_2.require::<FrameService>("frame_service");
        let mut frame_service = frame_service.write();

        frame_service.begin_frame();
    });
}

pub fn platform(
    resource_container: Arc<ResourceContainer>,
    initializer: Initializer,
    settings: &Settings,
) {
    // Get dependencies
    let system_service = resource_container.require::<SystemService>("system_service");

    // Read settings
    let window_settings = read_window_settings(settings);

    // Build the window
    let event_loop = EventLoop::<()>::with_user_event();
    let window = WindowBuilder::new()
        .with_title(window_settings.title)
        .with_inner_size(LogicalSize::new(
            window_settings.width as u32,
            window_settings.height as u32,
        ))
        .with_resizable(window_settings.resizable)
        .build(&event_loop)
        .unwrap();

    let window_id = window.id();

    // Build and inject the windows service
    let window_service = WinitWindowService::new(window);

    let on_start_update = window_service.on_start_update.clone();
    let on_end_update = window_service.on_end_update.clone();
    let on_resize = window_service.on_resize.clone();
    let on_cursor_moved = window_service.on_cursor_moved.clone();
    let on_event = window_service.on_event.clone();
    let on_events_cleared = window_service.on_events_cleared.clone();

    resource_container
        .add::<dyn WindowService>("window_service", Box::new(window_service))
        .unwrap();

    // Initialize the engine
    initializer(resource_container.clone(), settings);

    // Run the begin systems before everything
    let system_service = system_service.clone();
    let system_service_reader = system_service.read();
    system_service_reader.run_begin();
    std::mem::drop(system_service_reader);

    // Run the render loop
    let window_service = resource_container.require::<dyn WindowService>("window_service");
    let window_service_reader = window_service.read();
    window_service_reader.on_enter_loop().notify(());
    std::mem::drop(window_service_reader);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Handle events
        {
            // TODO: Try to find a way to remove this
            let event = &event as *const _ as *const c_void;
            let event = event as *const Event<'static, ()>;
            let event = unsafe { &*event as &Event<'static, ()> };
            let event = unsafe { &*(&event as *const _) } as &Event<'static, ()>;
            let event = event.clone();
            on_event.notify(event);
        }

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
            // Check if the user has moved the cursor
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                let mut window_service = window_service.write();
                let mut window_service = window_service
                    .as_any_mut()
                    .downcast_mut::<WinitWindowService>()
                    .unwrap();

                window_service.cursor_position = (position.x as usize, position.y as usize);
                std::mem::drop(window_service);

                on_cursor_moved.notify((position.x as usize, position.y as usize));
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
            Event::MainEventsCleared => {
                on_events_cleared.notify(());
            }
            _ => (),
        }

        // Start updating
        on_start_update.notify(());

        // Run the systems
        let system_service_reader = system_service.read();
        system_service_reader.run();

        // End the update
        on_end_update.notify(());
    });
}

fn read_window_settings(settings: &Settings) -> WindowSettings {
    let settings = settings.get_settings("window");

    WindowSettings {
        title: settings.get("title", "".to_string()),
        width: settings.get("width", 512),
        height: settings.get("height", 512),
        resizable: settings.get("resizable", true),
    }
}
