use crate::windows_manager::WindowsManager;
use core::ffi::c_void;
use fruity_core::platform::Initializer;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;
use std::sync::RwLock;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub mod windows_manager;

pub fn platform(
    service_manager: &Arc<RwLock<ServiceManager>>,
    initializer: Initializer,
    settings: &Settings,
) {
    /*let (run_callback, windows_manager) = WindowsManager::new(service_manager);

    let mut service_manager = service_manager.write().unwrap();
    service_manager.register("windows_manager", windows_manager);

    Some(run_callback)*/

    let service_manager_reader = service_manager.read().unwrap();
    let system_manager = service_manager_reader.get::<SystemManager>().unwrap();
    std::mem::drop(service_manager_reader);

    // Build the window
    let event_loop = EventLoop::<()>::with_user_event();
    let window = WindowBuilder::new()
        .with_title("Hit space to toggle resizability.")
        .with_inner_size(LogicalSize::new(800, 800))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();
    let window_id = window.id();

    // Build and inject the windows service
    let mut service_manager_writer = service_manager.write().unwrap();
    let windows_manager = WindowsManager::new(window);

    let on_start_update = windows_manager.on_start_update.clone();
    let on_end_update = windows_manager.on_end_update.clone();
    let on_resize = windows_manager.on_resize.clone();
    let on_cursor_moved = windows_manager.on_cursor_moved.clone();
    let on_event = windows_manager.on_event.clone();
    let on_events_cleared = windows_manager.on_events_cleared.clone();

    service_manager_writer.register("windows_manager", windows_manager);
    std::mem::drop(service_manager_writer);

    // Initialize the engine
    initializer(service_manager, settings);

    // Run the begin systems before everything
    let system_manager = system_manager.clone();
    let system_manager_reader = system_manager.read().unwrap();
    system_manager_reader.run_begin();
    std::mem::drop(system_manager_reader);

    // Run the render loop

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
        let system_manager_reader = system_manager.read().unwrap();
        system_manager_reader.run();

        // End the update
        on_end_update.notify(());
    });
}
