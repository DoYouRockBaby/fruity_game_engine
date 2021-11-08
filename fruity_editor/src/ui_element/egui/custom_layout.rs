pub fn flex_row_layout(
    ui: &mut egui::Ui,
    add_contents: &[impl FnOnce(&mut egui::Ui],
) -> egui::Response {
    // Build each item cell
    let cell_size = egui::Vec2::new(item_width, item_height);
    let available_width = ui.available_size().x;
    let item_per_row = (available_width / item_width).floor() as usize;
    let line_count = item_count / item_per_row;

    // Allocate size for the widget
    let desired_size = egui::vec2(
        item_per_row as f32 * item_width,
        (line_count as f32) * item_height,
    );

    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let mut children = (0..item_count)
        .map(|item_index| {
            let x_index = item_index % item_per_row;
            let y_index = item_index / item_per_row;
            let relative_pos =
                egui::Vec2::new(x_index as f32 * item_width, y_index as f32 * item_height);

            let child_rect = egui::Rect::from_min_size(rect.left_top() + relative_pos, cell_size);

            ui.child_ui(
                child_rect,
                egui::Layout::top_down_justified(egui::Align::LEFT),
            )
        })
        .collect::<Vec<_>>();

    // Build the children
    add_contents(&mut children);

    response
}
