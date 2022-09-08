use crate::state::scene::SceneState;
use crate::ui::context::UIContext;
use crate::ui::elements::input::Button;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use crate::ui::hooks::use_read_service;
use crate::ui::hooks::use_write_service;
use std::sync::Arc;

pub fn run_controls_component(ctx: &mut UIContext) -> Vec<UIElement> {
    let scene_state = use_read_service::<SceneState>(ctx);

    vec![
        if !scene_state.is_running() {
            Button {
                label: "▶".to_string(),
                on_click: Arc::new(move |ctx| {
                    let mut scene_state = use_write_service::<SceneState>(&ctx);
                    scene_state.run();
                }),
                ..Default::default()
            }
            .elem()
        } else {
            Button {
                label: "⏸".to_string(),
                on_click: Arc::new(move |ctx| {
                    let mut scene_state = use_write_service::<SceneState>(&ctx);
                    scene_state.pause();
                }),
                ..Default::default()
            }
            .elem()
        },
        Button {
            label: "◼".to_string(),
            on_click: Arc::new(move |ctx| {
                let mut scene_state = use_write_service::<SceneState>(&ctx);
                scene_state.stop();
            }),
            enabled: scene_state.can_stop(),
            ..Default::default()
        }
        .elem(),
    ]
}
