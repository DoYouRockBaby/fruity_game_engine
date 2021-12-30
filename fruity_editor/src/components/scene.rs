use crate::ui_element::scene::Scene;
use crate::ui_element::UIElement;
use crate::ui_element::UIWidget;

pub fn scene_component() -> UIElement {
    Scene {}.elem()
}
