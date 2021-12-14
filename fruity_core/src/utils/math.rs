use std::f32::consts::PI;
use std::ops::Range;

pub fn normalise_angle(angle: f32) -> f32 {
    if angle < -PI {
        normalise_angle(angle + 2.0 * PI)
    } else if angle > PI {
        normalise_angle(angle - 2.0 * PI)
    } else {
        angle
    }
}

pub fn normalise_angle_range(original: Range<f32>) -> Range<f32> {
    let angle1 = normalise_angle(original.start);
    let angle2 = normalise_angle(original.end);

    let start = f32::min(angle1, angle2);
    let end = f32::max(angle1, angle2);

    if start == end {
        -PI..PI
    } else {
        start..end
    }
}
