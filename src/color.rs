#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        return Self { r, g, b, a: 1.0 };
    }
    pub fn new_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Self { r, g, b, a };
    }
    pub fn red() -> Self {
        Color::new(0.255, 0.0, 0.0)
    }
    pub fn green() -> Self {
        Color::new(0.0, 0.255, 0.0)
    }
    pub fn blue() -> Self {
        Color::new(0.0, 0.0, 0.255)
    }
}
