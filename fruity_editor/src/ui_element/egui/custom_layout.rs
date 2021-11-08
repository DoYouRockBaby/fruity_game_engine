use crate::hooks::topo;
use crate::hooks::use_state;
use crate::ui_element::egui::draw_element;
use crate::ui_element::egui::DrawContext;
use crate::ui_element::UIElement;
use comp_state::CloneState;
use std::collections::HashMap;

#[topo::nested]
pub fn flex_row_layout(
    children: Vec<UIElement>,
    ui: &mut egui::Ui,
    ctx: &mut DrawContext,
) -> egui::Response {
    // When a child overlap the available width, we will store it to remember next time
    // when to do a new-line
    let last_available_width = use_state(|| 0.0);
    let last_child_count = use_state(|| 0);
    let new_line_childs = use_state::<HashMap<usize, f32>, _>(|| HashMap::new());
    let child_count = children.len();

    // Get the base available space informations
    let available_width = ui.available_size_before_wrap().x;
    let origin_pos = ui.available_rect_before_wrap().left_top();

    // If the size have changed or new child has been added, we the child line break should be changes
    if available_width != last_available_width.get() {
        new_line_childs.set(HashMap::new());
    }

    if child_count != last_child_count.get() {
        new_line_childs.set(HashMap::new());
    }

    let mut allocated_rect = egui::Rect::from_min_size(origin_pos, egui::Vec2::new(0.0, 0.0));
    let mut relative_pos = egui::Vec2::new(0.0, 0.0);
    let default_size = ui.spacing().interact_size;
    let new_line_childs_value = new_line_childs.get();
    let mut current_line_height = 0.0;
    for (index, child) in children.into_iter().enumerate() {
        // If the child is newline, we proceed
        if let Some(line_height) = new_line_childs_value.get(&index) {
            relative_pos = egui::Vec2::new(
                0.0,
                relative_pos.y + line_height + ui.spacing().item_spacing.y,
            );
            current_line_height = 0.0;
        }

        // Build the base child rect
        let child_rect = egui::Rect::from_min_size(origin_pos + relative_pos, default_size);
        let mut child_ui = ui.child_ui(
            child_rect,
            egui::Layout::top_down_justified(egui::Align::LEFT),
        );

        // Draw the child
        draw_element(child, &mut child_ui, ctx);
        let final_child_rect = child_ui.min_rect();

        // We update the position where the next child will be rendered
        relative_pos +=
            egui::Vec2::new(final_child_rect.size().x + ui.spacing().item_spacing.x, 0.0);
        current_line_height = f32::max(final_child_rect.size().y, current_line_height);

        // If the child overlap, we remember it for next time
        if relative_pos.x > available_width {
            new_line_childs.update(|new_line_childs| {
                new_line_childs.insert(index, current_line_height);
            });

            break;
        } else {
            allocated_rect.max = egui::Pos2::new(
                f32::max(allocated_rect.max.x, final_child_rect.max.x),
                f32::max(allocated_rect.max.y, final_child_rect.max.y),
            );
        }
    }

    // Request the drawed rect to make egui aware and react about all the child
    let response = ui.allocate_rect(allocated_rect, egui::Sense::click());

    // Store the last width and last child count
    last_available_width.set(available_width);
    last_child_count.set(child_count);

    response
}
