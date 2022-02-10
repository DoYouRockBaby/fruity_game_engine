use fruity_core::inject::Const;
use fruity_ecs::system::system_service::SystemService;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;

pub fn display_grid(
    system_service: Const<SystemService>,
    graphic_service: Const<dyn GraphicService>,
    graphic_2d_service: Const<Graphic2dService>,
) {
    if system_service.is_paused() {
        let screen_bottom_left =
            graphic_service.get_camera_transform().invert() * Vector2d::new(-1.0, -1.0);
        let screen_top_right =
            graphic_service.get_camera_transform().invert() * Vector2d::new(1.0, 1.0);

        let x_begin = screen_bottom_left.x.trunc() as i32;
        let y_begin = screen_bottom_left.y.trunc() as i32;

        let x_end = screen_top_right.x.trunc() as i32 + 1;
        let y_end = screen_top_right.y.trunc() as i32 + 1;

        let x_line_count = x_end - x_begin;
        let y_line_count = y_end - y_begin;

        (0..x_line_count).for_each(|x| {
            graphic_2d_service.draw_line(
                Vector2d::new((x_begin + x) as f32, screen_bottom_left.y),
                Vector2d::new((x_begin + x) as f32, screen_top_right.y),
                1,
                Color::white(),
                -10,
            )
        });

        (0..y_line_count).for_each(|y| {
            graphic_2d_service.draw_line(
                Vector2d::new(screen_bottom_left.x, (y_begin + y) as f32),
                Vector2d::new(screen_top_right.x, (y_begin + y) as f32),
                1,
                Color::white(),
                -10,
            )
        });
    }
}
