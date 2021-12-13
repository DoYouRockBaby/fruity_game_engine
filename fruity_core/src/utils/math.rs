use std::f32::consts::PI;

pub fn normalise_angle(angle: f32) -> f32 {
    if angle <= -PI {
        normalise_angle(angle + 2.0 * PI)
    } else if angle > PI {
        normalise_angle(angle - 2.0 * PI)
    } else {
        angle
    }
}
