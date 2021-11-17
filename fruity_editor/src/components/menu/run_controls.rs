use crate::hooks::use_global;
use crate::state::scene::SceneState;
use crate::ui_element::input::Button;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;
use std::sync::Arc;

pub fn run_controls_component() -> Vec<UIElement> {
    let scene_state = use_global::<SceneState>();

    vec![
        if !scene_state.is_running() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move || {
                    let scene_state = use_global::<SceneState>();
                    scene_state.run();
                }),
                ..Default::default()
            }
            .elem()
        } else {
            Button {
                label: "⏸".to_string(),
                on_click: Arc::new(move || {
                    let scene_state = use_global::<SceneState>();
                    scene_state.pause();
                }),
                ..Default::default()
            }
            .elem()
        },
        Button {
            label: "◼".to_string(),
            on_click: Arc::new(move || {
                let scene_state = use_global::<SceneState>();
                scene_state.stop();
            }),
            enabled: scene_state.can_stop(),
            ..Default::default()
        }
        .elem(),
    ]
}
