use crate::ui::context::UIContext;
use crate::ui::elements::scene::Scene;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;

pub fn scene_component(_ctx: &mut UIContext) -> UIElement {
    Scene {}.elem()
}
