use fruity_core::inject::Const;
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::math::Color;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_physic_2d_rapier::rapier_2d_service::Rapier2dService;
use rapier2d::prelude::Ball;
use rapier2d::prelude::Cuboid;

pub fn draw_physic_debug(
    rapier_2d_service: Const<Rapier2dService>,
    graphic_2d_service: Const<Graphic2dService>,
) {
    rapier_2d_service
        .collider_set
        .iter()
        .map(|(_, collider)| collider)
        .for_each(|collider| {
            if let Some(shape) = collider.shape().downcast_ref::<Cuboid>() {
                let translation = collider.translation();
                let rotation = collider.rotation();
                let transform =
                    Matrix3::new_translation(Vector2d::new(translation.x, translation.y))
                        * Matrix3::new_rotation(rotation.angle());

                let mut polyline = shape
                    .to_polyline()
                    .into_iter()
                    .map(|point| transform * Vector2d::new(point.coords.x, point.coords.y))
                    .collect::<Vec<_>>();

                if polyline.len() > 0 {
                    polyline.push(polyline.first().unwrap().clone());
                    graphic_2d_service.draw_polyline(polyline, 3, Color::blue(), 999);
                }
            } else if let Some(shape) = collider.shape().downcast_ref::<Ball>() {
                let translation = collider.translation();
                let rotation = collider.rotation();
                let transform =
                    Matrix3::new_translation(Vector2d::new(translation.x, translation.y))
                        * Matrix3::new_rotation(rotation.angle());
                let center = transform * Vector2d::new(0.0, 0.0);
                let radius = shape.radius;

                graphic_2d_service.draw_circle(
                    Vector2d::new(center.x, center.y),
                    radius,
                    3,
                    Color::alpha(),
                    Color::blue(),
                    999,
                );
            }
        });
}
