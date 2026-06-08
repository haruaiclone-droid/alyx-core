use alyx_core::ir::{Align, FlexDirection, FlexLayout, Justify, Layout, Padding};
use alyx_core::widgets::*;

fn main() {
    let layout = Widget::<()>::Container(ContainerWidget {
        children: vec![
            Widget::<()>::Text(TextWidget::new("title").size(120.0, 24.0)),
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            Widget::Container(ContainerWidget {
                children: vec![
                    Widget::<()>::Text(TextWidget::new("left").size(40.0, 16.0)),
                    Widget::<()>::Text(TextWidget::new("right").size(40.0, 16.0)),
                ],
                layout: Layout::Flex(FlexLayout {
                    direction: FlexDirection::Row,
                    gap: 12.0,
                    padding: Padding::default(),
                    align: Align::Start,
                    justify: Justify::Start,
                }),
            }),
            Widget::<()>::Link(LinkWidget::new("docs", "/docs", None)),
        ],
        layout: Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 4.0,
            padding: Padding::default(),
            align: Align::Start,
            justify: Justify::Start,
        }),
    });

    let _ = layout.into_ir();
}
