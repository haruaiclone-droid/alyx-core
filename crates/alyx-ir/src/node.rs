use crate::{FlexDirection, ImageSource, ImageStyle, Layout, Rect, Size, TextStyle};

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
            Self::Container(container) => container.size(),
            Self::Text(text) => text.size,
            Self::Image(image) => image.style.size,
            Self::HitArea(hit_area) => hit_area.size(),
            Self::Pane(pane) => pane.size(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Container<Msg> {
    pub children: Vec<IrNode<Msg>>,
    pub layout: Layout,
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
    pub layout: Layout,
    pub child: Box<IrNode<Msg>>,
    pub on_click: Option<Msg>,
    pub on_hover: Option<Msg>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pane {
    pub rect: Rect,
}

impl<Msg> Container<Msg> {
    pub fn new(children: Vec<IrNode<Msg>>, layout: Layout) -> Self {
        Self { children, layout }
    }

    pub fn size(&self) -> Size {
        match self.layout {
            Layout::Flex(layout) => {
                let mut width = layout.padding.left + layout.padding.right;
                let mut height = layout.padding.top + layout.padding.bottom;

                if self.children.is_empty() {
                    return Size { width, height };
                }

                match layout.direction {
                    FlexDirection::Row => {
                        let mut content_width = 0.0;
                        let mut content_height: f32 = 0.0;

                        for (index, child) in self.children.iter().enumerate() {
                            let child_size = child.size();
                            content_width += child_size.width;
                            if index > 0 {
                                content_width += layout.gap;
                            }
                            content_height = content_height.max(child_size.height);
                        }

                        width += content_width;
                        height += content_height;
                    }
                    FlexDirection::Column => {
                        let mut content_width: f32 = 0.0;
                        let mut content_height = 0.0;

                        for (index, child) in self.children.iter().enumerate() {
                            let child_size = child.size();
                            content_width = content_width.max(child_size.width);
                            content_height += child_size.height;
                            if index > 0 {
                                content_height += layout.gap;
                            }
                        }

                        width += content_width;
                        height += content_height;
                    }
                }

                Size { width, height }
            }
        }
    }
}

impl<Msg> HitArea<Msg> {
    pub fn new(
        layout: Layout,
        child: IrNode<Msg>,
        on_click: Option<Msg>,
        on_hover: Option<Msg>,
    ) -> Self {
        Self {
            layout,
            child: Box::new(child),
            on_click,
            on_hover,
        }
    }

    pub fn size(&self) -> Size {
        match self.layout {
            Layout::Flex(layout) => {
                let child_size = self.child.size();
                Size {
                    width: layout.padding.left + child_size.width + layout.padding.right,
                    height: layout.padding.top + child_size.height + layout.padding.bottom,
                }
            }
        }
    }

    pub fn child_origin(&self, x: f32, y: f32) -> (f32, f32) {
        match self.layout {
            Layout::Flex(layout) => (x + layout.padding.left, y + layout.padding.top),
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
