use alyx_ir::{
    AccessibilityMetadata, Align, Color, Container, FlexDirection, FlexLayout, Font, HitArea,
    Image, ImageSource, ImageStyle, IrNode, Justify, Layout, Padding, Pane, Rect, Role, Size, Text,
    TextStyle,
};
use std::marker::PhantomData;

pub trait IntoIr<Msg> {
    fn into_ir(self) -> IrNode<Msg>;
}

#[derive(Clone, Debug)]
pub struct ButtonBuilder<Msg> {
    label: String,
    _marker: PhantomData<Msg>,
}

impl<Msg: Clone> ButtonBuilder<Msg> {
    pub fn on_click(self, msg: Msg) -> Widget<Msg> {
        Widget::Button(ButtonWidget::text(self.label, msg))
    }
}

pub fn button<Msg: Clone>(label: impl Into<String>) -> ButtonBuilder<Msg> {
    ButtonBuilder {
        label: label.into(),
        _marker: PhantomData,
    }
}

pub fn text(content: impl Into<String>) -> TextWidget {
    TextWidget::new(content)
}

pub fn row<Msg: Clone>(children: impl IntoIterator<Item = Widget<Msg>>) -> Row<Msg> {
    Row::new(children.into_iter().collect())
}

pub fn column<Msg: Clone>(children: impl IntoIterator<Item = Widget<Msg>>) -> Column<Msg> {
    Column::new(children.into_iter().collect())
}

#[derive(Clone, Debug)]
pub struct TextWidget {
    pub content: String,
    pub size: f32,
    pub color: Color,
    pub font: String,
    pub width: f32,
    pub height: f32,
}

impl TextWidget {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            font: "system-ui".to_string(),
            width: 100.0,
            height: 20.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.color = Color { r, g, b, a };
        self
    }

    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.font = font.into();
        self
    }
}

#[derive(Clone, Debug)]
pub struct LinkWidget<Msg> {
    pub label: TextWidget,
    pub href: String,
    pub on_activate: Option<Msg>,
}

#[derive(Clone, Debug)]
pub struct CheckboxWidget<Msg> {
    pub label: String,
    pub checked: bool,
    pub on_toggle: Option<Msg>,
}

#[derive(Clone, Debug)]
pub struct TextInputWidget<Msg> {
    pub value: String,
    pub placeholder: String,
    pub on_change: Option<Msg>,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug)]
pub struct ListWidget<Msg> {
    pub items: Vec<Widget<Msg>>,
    pub gap: f32,
}

#[derive(Clone, Debug)]
pub struct ScrollWidget<Msg> {
    pub child: Box<Widget<Msg>>,
    pub direction: FlexDirection,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug)]
pub struct ButtonWidget<Msg> {
    pub label: TextWidget,
    pub on_click: Option<Msg>,
}

#[derive(Clone, Debug)]
pub struct Row<Msg> {
    pub children: Vec<Widget<Msg>>,
    pub gap: f32,
    pub padding: Padding,
}

#[derive(Clone, Debug)]
pub struct Column<Msg> {
    pub children: Vec<Widget<Msg>>,
    pub gap: f32,
    pub padding: Padding,
}

#[derive(Clone, Debug)]
pub struct Stack<Msg> {
    pub children: Vec<Widget<Msg>>,
    pub gap: f32,
}

#[derive(Clone, Debug)]
pub enum Widget<Msg> {
    Text(TextWidget),
    Button(ButtonWidget<Msg>),
    Link(LinkWidget<Msg>),
    Checkbox(CheckboxWidget<Msg>),
    TextInput(TextInputWidget<Msg>),
    List(ListWidget<Msg>),
    Scroll(ScrollWidget<Msg>),
    Stack(Stack<Msg>),
    Custom(IrNode<Msg>),
    Image(ImageWidget),
    Container(ContainerWidget<Msg>),
    Spacer { width: f32, height: f32 },
    Pane(Rect),
}

#[derive(Clone, Debug)]
pub struct ImageWidget {
    pub source: ImageSource,
    pub size: Size,
}

#[derive(Clone, Debug)]
pub struct ContainerWidget<Msg> {
    pub children: Vec<Widget<Msg>>,
    pub layout: Layout,
}

impl TextWidget {
    pub fn to_node<Msg: Clone>(self) -> IrNode<Msg> {
        IrNode::Text(Text {
            content: self.content,
            style: TextStyle {
                font: Font { family: self.font },
                size: self.size,
                color: self.color,
            },
            size: Size {
                width: self.width,
                height: self.height,
            },
        })
    }
}

impl<Msg: Clone> LinkWidget<Msg> {
    pub fn new(
        label: impl Into<String>,
        href: impl Into<String>,
        on_activate: Option<Msg>,
    ) -> Self {
        Self {
            label: TextWidget::new(label),
            href: href.into(),
            on_activate,
        }
    }
}

impl<Msg: Clone> CheckboxWidget<Msg> {
    pub fn new(label: impl Into<String>, checked: bool, on_toggle: Option<Msg>) -> Self {
        Self {
            label: label.into(),
            checked,
            on_toggle,
        }
    }
}

impl<Msg: Clone> TextInputWidget<Msg> {
    pub fn new(value: impl Into<String>, on_change: Option<Msg>) -> Self {
        Self {
            value: value.into(),
            placeholder: String::new(),
            on_change,
            width: 120.0,
            height: 20.0,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl<Msg> ListWidget<Msg> {
    pub fn new(items: Vec<Widget<Msg>>) -> Self {
        Self { items, gap: 8.0 }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl<Msg> ScrollWidget<Msg> {
    pub fn vertical(child: Widget<Msg>) -> Self {
        Self {
            child: Box::new(child),
            direction: FlexDirection::Column,
            width: 320.0,
            height: 240.0,
        }
    }

    pub fn horizontal(child: Widget<Msg>) -> Self {
        Self {
            child: Box::new(child),
            direction: FlexDirection::Row,
            width: 320.0,
            height: 240.0,
        }
    }
}

impl<Msg: Clone> Stack<Msg> {
    pub fn new(children: Vec<Widget<Msg>>) -> Self {
        Self { children, gap: 0.0 }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl<Msg: Clone> ButtonWidget<Msg> {
    pub fn text(content: impl Into<String>, on_click: Msg) -> Self {
        Self {
            label: TextWidget::new(content),
            on_click: Some(on_click),
        }
    }

    pub fn to_node(self) -> IrNode<Msg> {
        IrNode::HitArea(
            HitArea::new(
                Layout::Flex(FlexLayout {
                    direction: FlexDirection::Row,
                    gap: 0.0,
                    padding: Padding::default(),
                    align: Align::Center,
                    justify: alyx_ir::Justify::Center,
                }),
                self.label.to_node(),
                self.on_click.clone(),
                None,
            )
            .accessibility(
                AccessibilityMetadata::new()
                    .role(Role::Button)
                    .focusable(true),
            ),
        )
    }
}

impl<Msg: Clone> Row<Msg> {
    pub fn new(children: Vec<Widget<Msg>>) -> Self {
        Self {
            children,
            gap: 0.0,
            padding: Padding::default(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.padding = Padding {
            left,
            top,
            right,
            bottom,
        };
        self
    }
}

impl<Msg: Clone> Column<Msg> {
    pub fn new(children: Vec<Widget<Msg>>) -> Self {
        Self {
            children,
            gap: 0.0,
            padding: Padding::default(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn padding(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        self.padding = Padding {
            left,
            top,
            right,
            bottom,
        };
        self
    }
}

impl<Msg: Clone> From<Row<Msg>> for Widget<Msg> {
    fn from(row: Row<Msg>) -> Self {
        Widget::Container(ContainerWidget {
            children: row.children,
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Row,
                gap: row.gap,
                padding: row.padding,
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> From<Column<Msg>> for Widget<Msg> {
    fn from(column: Column<Msg>) -> Self {
        Widget::Container(ContainerWidget {
            children: column.children,
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: column.gap,
                padding: column.padding,
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> ContainerWidget<Msg> {
    pub fn row(children: Vec<Widget<Msg>>) -> Self {
        Self {
            children,
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Row,
                gap: 0.0,
                padding: Padding::default(),
                align: Align::Start,
                justify: alyx_ir::Justify::Start,
            }),
        }
    }

    pub fn column(children: Vec<Widget<Msg>>) -> Self {
        Self {
            children,
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: 0.0,
                padding: Padding::default(),
                align: Align::Start,
                justify: alyx_ir::Justify::Start,
            }),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        match &mut self.layout {
            Layout::Flex(flex) => flex.gap = gap,
        }
        self
    }

    pub fn with_padding(mut self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        match &mut self.layout {
            Layout::Flex(flex) => {
                flex.padding = Padding {
                    left,
                    top,
                    right,
                    bottom,
                };
            }
        }
        self
    }
}

impl ImageWidget {
    pub fn new(source: ImageSource) -> Self {
        Self {
            source,
            size: Size {
                width: 24.0,
                height: 24.0,
            },
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size { width, height };
        self
    }
}

impl<Msg: Clone> IntoIr<Msg> for Widget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        match self {
            Widget::Text(text) => text.to_node(),
            Widget::Button(button) => button.to_node(),
            Widget::Link(link) => link.into_ir(),
            Widget::Checkbox(checkbox) => checkbox.into_ir(),
            Widget::TextInput(input) => input.into_ir(),
            Widget::List(list) => list.into_ir(),
            Widget::Scroll(scroll) => scroll.into_ir(),
            Widget::Stack(stack) => stack.into_ir(),
            Widget::Custom(node) => node,
            Widget::Image(image) => IrNode::Image(Image {
                src: image.source,
                style: ImageStyle { size: image.size },
            }),
            Widget::Container(container) => IrNode::Container(Container {
                children: container
                    .children
                    .into_iter()
                    .map(IntoIr::into_ir)
                    .collect(),
                layout: container.layout,
            }),
            Widget::Spacer { width, height } => IrNode::Pane(Pane {
                rect: Rect {
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                },
            }),
            Widget::Pane(rect) => IrNode::Pane(Pane { rect }),
        }
    }
}

impl<Msg: Clone> IntoIr<Msg> for LinkWidget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        let label = self.label.to_node();
        IrNode::HitArea(
            HitArea::new(
                Layout::Flex(FlexLayout {
                    direction: FlexDirection::Row,
                    gap: 0.0,
                    padding: Padding::default(),
                    align: Align::Start,
                    justify: Justify::Start,
                }),
                label,
                self.on_activate,
                None,
            )
            .accessibility(
                AccessibilityMetadata::new()
                    .role(Role::Link)
                    .focusable(true),
            )
            .navigate_to(self.href),
        )
    }
}

impl<Msg: Clone> IntoIr<Msg> for CheckboxWidget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        let content = if self.checked {
            format!("[x] {}", self.label)
        } else {
            format!("[ ] {}", self.label)
        };
        IrNode::HitArea(
            HitArea::new(
                Layout::Flex(FlexLayout {
                    direction: FlexDirection::Row,
                    gap: 0.0,
                    padding: Padding::default(),
                    align: Align::Center,
                    justify: Justify::Start,
                }),
                IrNode::Text(Text {
                    content,
                    style: TextStyle {
                        font: Font {
                            family: "system-ui".to_string(),
                        },
                        size: 14.0,
                        color: Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    },
                    size: Size {
                        width: 120.0,
                        height: 20.0,
                    },
                }),
                self.on_toggle,
                None,
            )
            .accessibility(
                AccessibilityMetadata::new()
                    .role(Role::Checkbox)
                    .focusable(true),
            ),
        )
    }
}

impl<Msg: Clone> IntoIr<Msg> for TextInputWidget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        let shown = if self.value.is_empty() {
            self.placeholder
        } else {
            self.value
        };
        let mut hit_area = HitArea::new(
            Layout::Flex(FlexLayout {
                direction: FlexDirection::Row,
                gap: 0.0,
                padding: Padding {
                    left: 4.0,
                    top: 2.0,
                    right: 4.0,
                    bottom: 2.0,
                },
                align: Align::Start,
                justify: Justify::Start,
            }),
            IrNode::Text(Text {
                content: shown,
                style: TextStyle {
                    font: Font {
                        family: "system-ui".to_string(),
                    },
                    size: 14.0,
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                },
                size: Size {
                    width: self.width,
                    height: self.height,
                },
            }),
            None,
            None,
        )
        .accessibility(
            AccessibilityMetadata::new()
                .role(Role::InputText)
                .focusable(true),
        );

        if let Some(msg) = self.on_change {
            hit_area = hit_area.key_down(msg);
        }

        IrNode::HitArea(hit_area)
    }
}

impl<Msg: Clone> IntoIr<Msg> for ListWidget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        IrNode::Container(Container {
            children: self.items.into_iter().map(IntoIr::into_ir).collect(),
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: self.gap,
                padding: Padding::default(),
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> IntoIr<Msg> for ScrollWidget<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        IrNode::Container(Container {
            children: vec![self.child.into_ir()],
            layout: Layout::Flex(FlexLayout {
                direction: self.direction,
                gap: 0.0,
                padding: Padding::default(),
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> IntoIr<Msg> for Stack<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        IrNode::Container(Container {
            children: self.children.into_iter().map(IntoIr::into_ir).collect(),
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: self.gap,
                padding: Padding::default(),
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> IntoIr<Msg> for Row<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        IrNode::Container(Container {
            children: self.children.into_iter().map(IntoIr::into_ir).collect(),
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Row,
                gap: self.gap,
                padding: self.padding,
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}

impl<Msg: Clone> IntoIr<Msg> for Column<Msg> {
    fn into_ir(self) -> IrNode<Msg> {
        IrNode::Container(Container {
            children: self.children.into_iter().map(IntoIr::into_ir).collect(),
            layout: Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: self.gap,
                padding: self.padding,
                align: Align::Start,
                justify: Justify::Start,
            }),
        })
    }
}
