use alyx_core::widgets::*;

fn main() {
    let layout: alyx_core::ir::IrNode<()> = column::<()>([
        Widget::Text(text("title").size(120.0, 24.0)),
        Widget::Spacer {
            width: 0.0,
            height: 8.0,
        },
        row([
            Widget::Text(text("left").size(40.0, 16.0)),
            Widget::Text(text("right").size(40.0, 16.0)),
        ])
        .gap(12.0)
        .into(),
        Widget::Link(LinkWidget::new("docs", "/docs", None)),
    ])
    .gap(4.0)
    .into_ir();

    let _ = layout;
}
