use alyx_ir::{ImageSource, TextStyle};

#[derive(Clone, Debug, PartialEq)]
pub struct RenderingPlan {
    pub nodes: Vec<RpNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RpNode {
    Container(RpContainer),
    Text(RpText),
    Image(RpImage),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RpContainer {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub children: Vec<RpNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RpText {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub content: String,
    pub style: TextStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RpImage {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub src: ImageSource,
}
