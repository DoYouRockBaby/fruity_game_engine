use crate::windows_manager::WinitWindowsManager;
use core::ffi::c_void;
use fruity_core::platform::Initializer;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use fruity_windows::frame_manager::FrameManager;
use fruity_windows::windows_manager::WindowsManager;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub mod windows_manager;

struct WindowSettings {
    title: String,
    width: usize,
    height: usize,
    resizable: bool,
}

pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let windows_manager = resource_manager.require::<dyn WindowsManager>("windows_manager");
    let windows_manager = windows_manager.read();
    let windows_manager = windows_manager
        .as_any_ref()
        .downcast_ref::<WinitWindowsManager>()
        .unwrap();

    let resource_manager_2 = resource_manager.clone();
    windows_manager.on_enter_loop.add_observer(move |_| {
        let frame_manager = resource_manager_2.require::<FrameManager>("frame_manager");
        let mut frame_manager = frame_manager.write();

        frame_manager.begin_frame();
    });

    let resource_manager_2 = resource_manager.clone();
    windows_manager.on_start_update.add_observer(move |_| {
        let frame_manager = resource_manager_2.require::<FrameManager>("frame_manager");
        let mut frame_manager = frame_manager.write();

        frame_manager.begin_frame();
    });
}

pub fn platform(
    resource_manager: Arc<ResourceManager>,
    initializer: Initializer,
    settings: &Settings,
) {
    // Get dependencies
    let system_manager = resource_manager.require::<SystemManager>("system_manager");

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
    let windows_manager = WinitWindowsManager::new(window);

    let on_start_update = windows_manager.on_start_update.clone();
    let on_end_update = windows_manager.on_end_update.clone();
    let on_resize = windows_manager.on_resize.clone();
    let on_cursor_moved = windows_manager.on_cursor_moved.clone();
    let on_event = windows_manager.on_event.clone();
    let on_events_cleared = windows_manager.on_events_cleared.clone();

    resource_manager
        .add::<dyn WindowsManager>("windows_manager", Box::new(windows_manager))
        .unwrap();

    // Initialize the engine
    initializer(resource_manager.clone(), settings);

    // Run the begin systems before everything
    let system_manager = system_manager.clone();
    let system_manager_reader = system_manager.read();
    system_manager_reader.run_begin();
    std::mem::drop(system_manager_reader);

    // Run the render loop
    let windows_manager = resource_manager.require::<dyn WindowsManager>("windows_manager");
    let windows_manager_reader = windows_manager.read();
    windows_manager_reader.on_enter_loop().notify(());
    std::mem::drop(windows_manager_reader);

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
                let mut windows_manager = windows_manager.write();
                let mut windows_manager = windows_manager
                    .as_any_mut()
                    .downcast_mut::<WinitWindowsManager>()
                    .unwrap();

                windows_manager.cursor_position = (position.x as usize, position.y as usize);
                std::mem::drop(windows_manager);

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
        let system_manager_reader = system_manager.read();
        system_manager_reader.run();

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
