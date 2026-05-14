use crate::Size;

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    pub font: Font,
    pub size: f32,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Font {
    pub family: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageStyle {
    pub size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
