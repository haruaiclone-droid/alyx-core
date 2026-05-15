use crate::{ImageSource, ImageStyle, Layout, Rect, Size, TextStyle};

#[derive(Clone, Debug, PartialEq)]
pub enum IrNode<Msg> {
    Container(Container<Msg>),
    Text(Text),
    Image(Image),
    HitArea(HitArea<Msg>),
    Pane(Pane),
}

impl<Msg> IrNode<Msg> {
    pub fn size(&self) -> Size {
        match self {
            Self::Container(container) => container.size,
            Self::Text(text) => text.size,
            Self::Image(image) => image.style.size,
            Self::HitArea(hit_area) => hit_area.child.size(),
            Self::Pane(pane) => pane.size(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Container<Msg> {
    pub children: Vec<IrNode<Msg>>,
    pub layout: Layout,
    pub size: Size,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub content: String,
    pub style: TextStyle,
    pub size: Size,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub src: ImageSource,
    pub style: ImageStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HitArea<Msg> {
    pub rect: Rect,
    pub child: Box<IrNode<Msg>>,
    pub on_click: Option<Msg>,
    pub on_hover: Option<Msg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pane {
    pub rect: Rect,
}

impl<Msg> Container<Msg> {
    pub fn new(children: Vec<IrNode<Msg>>, layout: Layout, size: Size) -> Self {
        Self {
            children,
            layout,
            size,
        }
    }
}

impl<Msg> HitArea<Msg> {
    pub fn new(
        rect: Rect,
        child: IrNode<Msg>,
        on_click: Option<Msg>,
        on_hover: Option<Msg>,
    ) -> Self {
        Self {
            rect,
            child: Box::new(child),
            on_click,
            on_hover,
        }
    }
}

impl Pane {
    pub fn size(&self) -> Size {
        Size {
            width: self.rect.width,
            height: self.rect.height,
        }
    }
}
