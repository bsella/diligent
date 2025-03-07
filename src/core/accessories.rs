// https://en.wikipedia.org/wiki/SRGB
pub fn linear_to_gamma(x: f32) -> f32 {
    return if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * f32::powf(x, 1.0 / 2.4) - 0.055
    };
}

pub fn linear_to_srgba(rgba: [f32; 4]) -> [f32; 4] {
    return [
        linear_to_gamma(rgba[0]),
        linear_to_gamma(rgba[1]),
        linear_to_gamma(rgba[2]),
        rgba[3],
    ];
}
