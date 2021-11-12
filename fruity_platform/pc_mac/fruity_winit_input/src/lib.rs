use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_input::input_service::InputService;
use fruity_windows::window_service::WindowService;
use fruity_winit_windows::window_service::WinitWindowService;
use std::sync::Arc;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::MouseButton;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let window_service = resource_container.require::<dyn WindowService>("window_service");
    let window_service = window_service.read();
    let window_service = window_service.downcast_ref::<WinitWindowService>();

    let input_service = resource_container.require::<InputService>("input_service");

    let input_service_2 = input_service.clone();
    window_service.on_event.add_observer(move |event| {
        let mut input_service = input_service_2.write();
        handle_keyboard_input(&mut input_service, event);
    });

    let input_service_2 = input_service.clone();
    window_service.on_end_update.add_observer(move |_| {
        let mut input_service = input_service_2.write();
        input_service.handle_frame_end();
    });
}

fn handle_keyboard_input(input_service: &mut InputService, event: &Event<()>) {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::MouseInput { state, button, .. } = event {
            let source = match button {
                MouseButton::Left => "Mouse/Left",
                MouseButton::Right => "Mouse/Right",
                MouseButton::Middle => "Mouse/Middle",
                _ => "Mouse/Unknown",
            };

            // Detect if pressed or released
            if ElementState::Pressed == *state {
                input_service.notify_pressed(source);
            } else {
                input_service.notify_released(source);
            }
        } else if let WindowEvent::KeyboardInput { input, .. } = event {
            if let Some(key) = input.virtual_keycode {
                // Get the key source
                let source = match key {
                    VirtualKeyCode::Key1 => "Keyboard/1",
                    VirtualKeyCode::Key2 => "Keyboard/2",
                    VirtualKeyCode::Key3 => "Keyboard/3",
                    VirtualKeyCode::Key4 => "Keyboard/4",
                    VirtualKeyCode::Key5 => "Keyboard/5",
                    VirtualKeyCode::Key6 => "Keyboard/6",
                    VirtualKeyCode::Key7 => "Keyboard/7",
                    VirtualKeyCode::Key8 => "Keyboard/8",
                    VirtualKeyCode::Key9 => "Keyboard/9",
                    VirtualKeyCode::Key0 => "Keyboard/0",
                    VirtualKeyCode::A => "Keyboard/A",
                    VirtualKeyCode::B => "Keyboard/B",
                    VirtualKeyCode::C => "Keyboard/C",
                    VirtualKeyCode::D => "Keyboard/D",
                    VirtualKeyCode::E => "Keyboard/E",
                    VirtualKeyCode::F => "Keyboard/F",
                    VirtualKeyCode::G => "Keyboard/G",
                    VirtualKeyCode::H => "Keyboard/H",
                    VirtualKeyCode::I => "Keyboard/I",
                    VirtualKeyCode::J => "Keyboard/J",
                    VirtualKeyCode::K => "Keyboard/K",
                    VirtualKeyCode::L => "Keyboard/L",
                    VirtualKeyCode::M => "Keyboard/M",
                    VirtualKeyCode::N => "Keyboard/N",
                    VirtualKeyCode::O => "Keyboard/O",
                    VirtualKeyCode::P => "Keyboard/P",
                    VirtualKeyCode::Q => "Keyboard/Q",
                    VirtualKeyCode::R => "Keyboard/R",
                    VirtualKeyCode::S => "Keyboard/S",
                    VirtualKeyCode::T => "Keyboard/T",
                    VirtualKeyCode::U => "Keyboard/U",
                    VirtualKeyCode::V => "Keyboard/V",
                    VirtualKeyCode::W => "Keyboard/W",
                    VirtualKeyCode::X => "Keyboard/X",
                    VirtualKeyCode::Y => "Keyboard/Y",
                    VirtualKeyCode::Z => "Keyboard/Z",
                    VirtualKeyCode::Escape => "Keyboard/Escape",
                    VirtualKeyCode::F1 => "Keyboard/F1",
                    VirtualKeyCode::F2 => "Keyboard/F2",
                    VirtualKeyCode::F3 => "Keyboard/F3",
                    VirtualKeyCode::F4 => "Keyboard/F4",
                    VirtualKeyCode::F5 => "Keyboard/F5",
                    VirtualKeyCode::F6 => "Keyboard/F6",
                    VirtualKeyCode::F7 => "Keyboard/F7",
                    VirtualKeyCode::F8 => "Keyboard/F8",
                    VirtualKeyCode::F9 => "Keyboard/F9",
                    VirtualKeyCode::F10 => "Keyboard/F10",
                    VirtualKeyCode::F11 => "Keyboard/F11",
                    VirtualKeyCode::F12 => "Keyboard/F12",
                    VirtualKeyCode::F13 => "Keyboard/F13",
                    VirtualKeyCode::F14 => "Keyboard/F14",
                    VirtualKeyCode::F15 => "Keyboard/F15",
                    VirtualKeyCode::F16 => "Keyboard/F16",
                    VirtualKeyCode::F17 => "Keyboard/F17",
                    VirtualKeyCode::F18 => "Keyboard/F18",
                    VirtualKeyCode::F19 => "Keyboard/F19",
                    VirtualKeyCode::F20 => "Keyboard/F20",
                    VirtualKeyCode::F21 => "Keyboard/F21",
                    VirtualKeyCode::F22 => "Keyboard/F22",
                    VirtualKeyCode::F23 => "Keyboard/F23",
                    VirtualKeyCode::F24 => "Keyboard/F24",
                    VirtualKeyCode::Snapshot => "Keyboard/Snapshot",
                    VirtualKeyCode::Scroll => "Keyboard/Scroll",
                    VirtualKeyCode::Pause => "Keyboard/Pause",
                    VirtualKeyCode::Insert => "Keyboard/Insert",
                    VirtualKeyCode::Home => "Keyboard/Home",
                    VirtualKeyCode::Delete => "Keyboard/Delete",
                    VirtualKeyCode::End => "Keyboard/End",
                    VirtualKeyCode::PageDown => "Keyboard/PageDown",
                    VirtualKeyCode::PageUp => "Keyboard/PageUp",
                    VirtualKeyCode::Left => "Keyboard/Left",
                    VirtualKeyCode::Up => "Keyboard/Up",
                    VirtualKeyCode::Right => "Keyboard/Right",
                    VirtualKeyCode::Down => "Keyboard/Down",
                    VirtualKeyCode::Back => "Keyboard/Back",
                    VirtualKeyCode::Return => "Keyboard/Return",
                    VirtualKeyCode::Space => "Keyboard/Space",
                    VirtualKeyCode::Compose => "Keyboard/Compose",
                    VirtualKeyCode::Caret => "Keyboard/Caret",
                    VirtualKeyCode::Numlock => "Keyboard/Numlock",
                    VirtualKeyCode::Numpad0 => "Keyboard/Numpad0",
                    VirtualKeyCode::Numpad1 => "Keyboard/Numpad1",
                    VirtualKeyCode::Numpad2 => "Keyboard/Numpad2",
                    VirtualKeyCode::Numpad3 => "Keyboard/Numpad3",
                    VirtualKeyCode::Numpad4 => "Keyboard/Numpad4",
                    VirtualKeyCode::Numpad5 => "Keyboard/Numpad5",
                    VirtualKeyCode::Numpad6 => "Keyboard/Numpad6",
                    VirtualKeyCode::Numpad7 => "Keyboard/Numpad7",
                    VirtualKeyCode::Numpad8 => "Keyboard/Numpad8",
                    VirtualKeyCode::Numpad9 => "Keyboard/Numpad9",
                    VirtualKeyCode::NumpadAdd => "Keyboard/NumpadAdd",
                    VirtualKeyCode::NumpadDivide => "Keyboard/NumpadDivide",
                    VirtualKeyCode::NumpadDecimal => "Keyboard/NumpadDecimal",
                    VirtualKeyCode::NumpadComma => "Keyboard/NumpadComma",
                    VirtualKeyCode::NumpadEnter => "Keyboard/NumpadEnter",
                    VirtualKeyCode::NumpadEquals => "Keyboard/NumpadEquals",
                    VirtualKeyCode::NumpadMultiply => "Keyboard/NumpadMultiply",
                    VirtualKeyCode::NumpadSubtract => "Keyboard/NumpadSubtract",
                    VirtualKeyCode::AbntC1 => "Keyboard/AbntC1",
                    VirtualKeyCode::AbntC2 => "Keyboard/AbntC2",
                    VirtualKeyCode::Apostrophe => "Keyboard/Apostrophe",
                    VirtualKeyCode::Apps => "Keyboard/Apps",
                    VirtualKeyCode::Asterisk => "Keyboard/Asterisk",
                    VirtualKeyCode::At => "Keyboard/At",
                    VirtualKeyCode::Ax => "Keyboard/Ax",
                    VirtualKeyCode::Backslash => "Keyboard/Backslash",
                    VirtualKeyCode::Calculator => "Keyboard/Calculator",
                    VirtualKeyCode::Capital => "Keyboard/Capital",
                    VirtualKeyCode::Colon => "Keyboard/Colon",
                    VirtualKeyCode::Comma => "Keyboard/Comma",
                    VirtualKeyCode::Convert => "Keyboard/Convert",
                    VirtualKeyCode::Equals => "Keyboard/Equals",
                    VirtualKeyCode::Grave => "Keyboard/Grave",
                    VirtualKeyCode::Kana => "Keyboard/Kana",
                    VirtualKeyCode::Kanji => "Keyboard/Kanji",
                    VirtualKeyCode::LAlt => "Keyboard/LAlt",
                    VirtualKeyCode::LBracket => "Keyboard/LBracket",
                    VirtualKeyCode::LControl => "Keyboard/LControl",
                    VirtualKeyCode::LShift => "Keyboard/LShift",
                    VirtualKeyCode::LWin => "Keyboard/LWin",
                    VirtualKeyCode::Mail => "Keyboard/Mail",
                    VirtualKeyCode::MediaSelect => "Keyboard/MediaSelect",
                    VirtualKeyCode::MediaStop => "Keyboard/MediaStop",
                    VirtualKeyCode::Minus => "Keyboard/Minus",
                    VirtualKeyCode::Mute => "Keyboard/Mute",
                    VirtualKeyCode::MyComputer => "Keyboard/MyComputer",
                    VirtualKeyCode::NavigateForward => "Keyboard/NavigateForward",
                    VirtualKeyCode::NavigateBackward => "Keyboard/NavigateBackward",
                    VirtualKeyCode::NextTrack => "Keyboard/NextTrack",
                    VirtualKeyCode::NoConvert => "Keyboard/NoConvert",
                    VirtualKeyCode::Period => "Keyboard/Period",
                    VirtualKeyCode::PlayPause => "Keyboard/PlayPause",
                    VirtualKeyCode::Plus => "Keyboard/Plus",
                    VirtualKeyCode::Power => "Keyboard/Power",
                    VirtualKeyCode::PrevTrack => "Keyboard/PrevTrack",
                    VirtualKeyCode::RAlt => "Keyboard/RAlt",
                    VirtualKeyCode::RBracket => "Keyboard/RBracket",
                    VirtualKeyCode::RControl => "Keyboard/RControl",
                    VirtualKeyCode::RShift => "Keyboard/RShift",
                    VirtualKeyCode::RWin => "Keyboard/RWin",
                    VirtualKeyCode::Semicolon => "Keyboard/Semicolon",
                    VirtualKeyCode::Slash => "Keyboard/Slash",
                    VirtualKeyCode::Sleep => "Keyboard/Sleep",
                    VirtualKeyCode::Stop => "Keyboard/Stop",
                    VirtualKeyCode::Sysrq => "Keyboard/Sysrq",
                    VirtualKeyCode::Tab => "Keyboard/Tab",
                    VirtualKeyCode::Underline => "Keyboard/Underline",
                    VirtualKeyCode::Unlabeled => "Keyboard/Unlabeled",
                    VirtualKeyCode::VolumeDown => "Keyboard/VolumeDown",
                    VirtualKeyCode::VolumeUp => "Keyboard/VolumeUp",
                    VirtualKeyCode::Wake => "Keyboard/Wake",
                    VirtualKeyCode::WebBack => "Keyboard/WebBack",
                    VirtualKeyCode::WebFavorites => "Keyboard/WebFavorites",
                    VirtualKeyCode::WebForward => "Keyboard/WebForward",
                    VirtualKeyCode::WebHome => "Keyboard/WebHome",
                    VirtualKeyCode::WebRefresh => "Keyboard/WebRefresh",
                    VirtualKeyCode::WebSearch => "Keyboard/WebSearch",
                    VirtualKeyCode::WebStop => "Keyboard/WebStop",
                    VirtualKeyCode::Yen => "Keyboard/Yen",
                    VirtualKeyCode::Copy => "Keyboard/Copy",
                    VirtualKeyCode::Paste => "Keyboard/Paste",
                    VirtualKeyCode::Cut => "Keyboard/Cut",
                    _ => "Keyboard/Unknown",
                };

                // Detect if pressed or released
                if ElementState::Pressed == input.state {
                    input_service.notify_pressed(source);
                } else {
                    input_service.notify_released(source);
                }
            }
        }
    }
}
