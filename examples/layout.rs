use alyx_core::widgets::*;

fn main() {
    let layout = Widget::Container(ContainerWidget {
        children: vec![
            Widget::Text(TextWidget::new("title").size(120.0, 24.0)),
            Widget::Spacer {
                width: 0.0,
                height: 8.0,
            },
            Widget::Container(ContainerWidget {
                children: vec![
                    Widget::Text(TextWidget::new("left").size(40.0, 16.0)),
                    Widget::Text(TextWidget::new("right").size(40.0, 16.0)),
                ],
                layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
                    direction: alyx_ir::FlexDirection::Row,
                    gap: 12.0,
                    padding: alyx_ir::Padding::default(),
                    align: alyx_ir::Align::Start,
                    justify: alyx_ir::Justify::Start,
                }),
            }),
            Widget::Link(LinkWidget::new("docs", "/docs", None)),
        ],
        layout: alyx_ir::Layout::Flex(alyx_ir::FlexLayout {
            direction: alyx_ir::FlexDirection::Column,
            gap: 4.0,
            padding: alyx_ir::Padding::default(),
            align: alyx_ir::Align::Start,
            justify: alyx_ir::Justify::Start,
        }),
    });

    let _ = layout.into_ir();
}
