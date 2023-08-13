pub mod camera;
pub mod input_manager;
pub mod widgets;

pub fn default<T: Default>() -> T {
    Default::default()
}

pub fn smoothstep(x: f32, left: f32, right: f32) -> f32 {
    let x = ((x - left) / (right - left)).clamp(0.0, 1.0);

    x * x * x * (3.0 * x * (2.0 * x - 5.0) + 10.0)
}
