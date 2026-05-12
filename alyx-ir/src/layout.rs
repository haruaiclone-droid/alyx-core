use crate::Padding;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    Flex(FlexLayout),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexLayout {
    pub direction: FlexDirection,
    pub gap: f32,
    pub padding: Padding,
    pub align: Align,
    pub justify: Justify,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
}

impl FlexLayout {
    pub fn row() -> Self {
        Self {
            direction: FlexDirection::Row,
            gap: 0.0,
            padding: Padding::default(),
            align: Align::Start,
            justify: Justify::Start,
        }
    }

    pub fn column() -> Self {
        Self {
            direction: FlexDirection::Column,
            gap: 0.0,
            padding: Padding::default(),
            align: Align::Start,
            justify: Justify::Start,
        }
    }
}
