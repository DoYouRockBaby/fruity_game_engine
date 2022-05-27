use fruity_core::inject::Const;
use fruity_ecs::system::system_service::SystemService;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;

const MIN_GRID_SQUARE_SIZE: u32 = 10;

pub fn display_grid(
    system_service: Const<SystemService>,
    graphic_service: Const<dyn GraphicService>,
    graphic_2d_service: Const<Graphic2dService>,
) {
    let mut square_unit = 1.0;

    loop {
        let screen_bottom_left = graphic_service.get_camera_transform().invert()
            * Vector2d::new(-square_unit, -square_unit);
        let screen_top_right = graphic_service.get_camera_transform().invert()
            * Vector2d::new(square_unit, square_unit);
        let square_unit_size = f32::abs(screen_top_right.x - screen_bottom_left.x) as u32;

        if square_unit_size < MIN_GRID_SQUARE_SIZE {
            square_unit *= 2.0;
        } else if square_unit_size >= MIN_GRID_SQUARE_SIZE * 2 {
            square_unit /= 2.0;
        } else {
            break;
        }
    }

    display_grid_with_square_unit(
        square_unit,
        system_service,
        graphic_service,
        graphic_2d_service,
    );
}

pub fn display_grid_with_square_unit(
    square_unit: f32,
    system_service: Const<SystemService>,
    graphic_service: Const<dyn GraphicService>,
    graphic_2d_service: Const<Graphic2dService>,
) {
    if system_service.is_paused() {
        let screen_bottom_left = graphic_service.get_camera_transform().invert()
            * Vector2d::new(-square_unit, -square_unit);
        let screen_top_right = graphic_service.get_camera_transform().invert()
            * Vector2d::new(square_unit, square_unit);

        let x_begin = screen_bottom_left.x.trunc() as i32;
        let y_begin = screen_bottom_left.y.trunc() as i32;

        let x_end = screen_top_right.x.trunc() as i32 + 1;
        let y_end = screen_top_right.y.trunc() as i32 + 1;

        (x_begin..x_end).for_each(|x| {
            if x % 2 == 0 {
                graphic_2d_service.draw_line(
                    Vector2d::new(x as f32 / square_unit, screen_bottom_left.y / square_unit),
                    Vector2d::new(x as f32 / square_unit, screen_top_right.y / square_unit),
                    1,
                    Color::white(),
                    -10,
                );
            } else {
                graphic_2d_service.draw_dotted_line(
                    Vector2d::new(x as f32 / square_unit, screen_bottom_left.y / square_unit),
                    Vector2d::new(x as f32 / square_unit, screen_top_right.y / square_unit),
                    1,
                    Color::white(),
                    -10,
                );
            }
        });

        (y_begin..y_end).for_each(|y| {
            if y % 2 == 0 {
                graphic_2d_service.draw_line(
                    Vector2d::new(screen_bottom_left.x / square_unit, y as f32 / square_unit),
                    Vector2d::new(screen_top_right.x / square_unit, y as f32 / square_unit),
                    1,
                    Color::white(),
                    -10,
                );
            } else {
                graphic_2d_service.draw_dotted_line(
                    Vector2d::new(screen_bottom_left.x / square_unit, y as f32 / square_unit),
                    Vector2d::new(screen_top_right.x / square_unit, y as f32 / square_unit),
                    1,
                    Color::white(),
                    -10,
                );
            }
        });
    }
}
