use alyx_ir::{Align, FlexDirection, FlexLayout, Justify, Layout, Padding};
use alyx_widgets::{
    ButtonWidget, ContainerWidget, IntoIr, TextWidget, Widget, button, column, row,
};

#[derive(Clone, Debug, PartialEq)]
enum Msg {
    Save,
}

#[test]
fn button_legacy_constructor_and_builder_output_match() {
    let legacy = Widget::Button(ButtonWidget::text("legacy", Msg::Save));
    let modern = button("legacy").on_click(Msg::Save);

    assert_eq!(legacy.into_ir(), modern.into_ir());
}

#[test]
fn container_legacy_api_matches_row_builder_layout() {
    let legacy = Widget::Container(ContainerWidget {
        children: vec![
            Widget::<Msg>::Text(TextWidget::new("left")),
            Widget::<Msg>::Text(TextWidget::new("right")),
        ],
        layout: Layout::Flex(FlexLayout {
            direction: FlexDirection::Row,
            gap: 8.0,
            padding: Padding {
                left: 3.0,
                top: 2.0,
                right: 1.0,
                bottom: 4.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    });

    let modern = row(vec![
        Widget::Text(TextWidget::new("left")),
        Widget::Text(TextWidget::new("right")),
    ])
    .gap(8.0)
    .padding(3.0, 2.0, 1.0, 4.0);

    assert_eq!(legacy.into_ir(), modern.into_ir());
}

#[test]
fn container_legacy_api_matches_column_builder_layout() {
    let legacy = Widget::Container(ContainerWidget {
        children: vec![
            Widget::<Msg>::Text(TextWidget::new("one")),
            Widget::<Msg>::Text(TextWidget::new("two")),
        ],
        layout: Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 6.0,
            padding: Padding {
                left: 4.0,
                top: 4.0,
                right: 4.0,
                bottom: 4.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    });

    let modern = column(vec![
        Widget::Text(TextWidget::new("one")),
        Widget::Text(TextWidget::new("two")),
    ])
    .gap(6.0)
    .padding(4.0, 4.0, 4.0, 4.0);

    assert_eq!(legacy.into_ir(), modern.into_ir());
}
