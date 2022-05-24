use crate::Rotate2d;
use crate::Scale2d;
use crate::Transform2d;
use crate::Translate2d;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::with::WithOptional;
use fruity_ecs::entity::entity_query::Query;
use fruity_graphic::math::matrix3::Matrix3;

pub fn update_transform_2d(
    query: Query<(
        WithMut<Transform2d>,
        WithOptional<Translate2d>,
        WithOptional<Rotate2d>,
        WithOptional<Scale2d>,
    )>,
) {
    query.for_each(|(mut transform, translate_2d, rotate_2d, scale_2d)| {
        transform.transform = Matrix3::new_identity();

        if let Some(translate_2d) = translate_2d {
            transform.transform = transform.transform * Matrix3::new_translation(translate_2d.vec);
        }

        if let Some(rotate_2d) = rotate_2d {
            transform.transform = transform.transform * Matrix3::new_rotation(rotate_2d.angle);
        }

        if let Some(scale_2d) = scale_2d {
            transform.transform = transform.transform * Matrix3::new_scaling(scale_2d.vec);
        }
    })
}
