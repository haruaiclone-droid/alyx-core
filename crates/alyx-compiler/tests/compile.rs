use alyx_compiler::compile;
use alyx_ir::{
    Align, Color, Container, FlexDirection, FlexLayout, Font, HitArea, Image, ImageSource,
    ImageStyle, IrNode, Justify, Layout, Padding, Rect, Size, Text, TextStyle,
};
use alyx_plan::{EventType, RpNode, RpText};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Msg {
    Click,
    Hover,
}

fn text(content: &str, width: f32, height: f32) -> IrNode<Msg> {
    IrNode::Text(Text {
        content: content.to_string(),
        style: TextStyle {
            font: Font {
                family: "Sans".to_string(),
            },
            size: 14.0,
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        },
        size: Size { width, height },
    })
}

#[test]
fn compiles_text_to_rendering_plan() {
    let root = text("hello", 80.0, 20.0);

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    assert_eq!(output.rp.nodes.len(), 1);
    assert_eq!(output.ep.hit_areas.len(), 0);

    let RpNode::Text(rp_text) = &output.rp.nodes[0] else {
        panic!("expected text node");
    };

    assert_eq!(rp_text.x, 0.0);
    assert_eq!(rp_text.y, 0.0);
    assert_eq!(rp_text.width, 80.0);
    assert_eq!(rp_text.height, 20.0);
    assert_eq!(rp_text.content, "hello");
}

#[test]
fn resolves_column_flex_positions_with_gap() {
    let root = IrNode::Container(Container::new(
        vec![text("a", 20.0, 10.0), text("b", 20.0, 15.0)],
        Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 5.0,
            padding: Padding {
                left: 2.0,
                top: 3.0,
                right: 0.0,
                bottom: 0.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    ));

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    assert_eq!(output.rp.nodes.len(), 2);

    let RpNode::Text(first) = &output.rp.nodes[0] else {
        panic!("expected first text node");
    };
    let RpNode::Text(second) = &output.rp.nodes[1] else {
        panic!("expected second text node");
    };

    assert_eq!((first.x, first.y), (2.0, 3.0));
    assert_eq!((second.x, second.y), (2.0, 18.0));
}

#[test]
fn resolves_row_flex_positions_with_gap() {
    let root = IrNode::Container(Container::new(
        vec![text("a", 20.0, 10.0), text("b", 30.0, 15.0)],
        Layout::Flex(FlexLayout {
            direction: FlexDirection::Row,
            gap: 8.0,
            padding: Padding {
                left: 4.0,
                top: 6.0,
                right: 0.0,
                bottom: 0.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    ));

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    assert_eq!(output.rp.nodes.len(), 2);

    let RpNode::Text(first) = &output.rp.nodes[0] else {
        panic!("expected first text node");
    };
    let RpNode::Text(second) = &output.rp.nodes[1] else {
        panic!("expected second text node");
    };

    assert_eq!((first.x, first.y), (4.0, 6.0));
    assert_eq!((second.x, second.y), (32.0, 6.0));
}

#[test]
fn resolves_hit_area_and_handler_table() {
    let root = IrNode::Container(Container::new(
        vec![IrNode::HitArea(HitArea::new(
            Layout::Flex(FlexLayout {
                direction: FlexDirection::Column,
                gap: 0.0,
                padding: Padding {
                    left: 1.0,
                    top: 2.0,
                    right: 3.0,
                    bottom: 4.0,
                },
                align: Align::Start,
                justify: Justify::Start,
            }),
            text("button", 50.0, 20.0),
            Some(Msg::Click),
            Some(Msg::Hover),
        ))],
        Layout::Flex(FlexLayout::column()),
    ));

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    assert_eq!(output.ep.hit_areas.len(), 2);
    assert_eq!(output.handlers.len(), 2);
    assert_eq!(output.rp.nodes.len(), 1);

    let RpNode::Text(RpText { x, y, .. }) = &output.rp.nodes[0] else {
        panic!("expected text node");
    };
    assert_eq!((*x, *y), (1.0, 2.0));

    let click_area = &output.ep.hit_areas[0];
    let hover_area = &output.ep.hit_areas[1];

    assert_eq!(click_area.event_type, EventType::Click);
    assert_eq!(hover_area.event_type, EventType::Hover);
    assert_eq!(
        click_area.rect,
        Rect {
            x: 0.0,
            y: 0.0,
            width: 54.0,
            height: 26.0
        }
    );
    assert_eq!(
        output.handlers.get(click_area.handler_id),
        Some(&Msg::Click)
    );
    assert_eq!(
        output.handlers.get(hover_area.handler_id),
        Some(&Msg::Hover)
    );
}

#[test]
fn resolves_container_size_from_children() {
    let nested = IrNode::Container(Container::new(
        vec![text("a", 20.0, 10.0), text("b", 30.0, 15.0)],
        Layout::Flex(FlexLayout {
            direction: FlexDirection::Column,
            gap: 5.0,
            padding: Padding {
                left: 4.0,
                top: 6.0,
                right: 3.0,
                bottom: 2.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
    ));

    let root = IrNode::Container(Container::new(
        vec![nested, text("c", 10.0, 5.0)],
        Layout::Flex(FlexLayout {
            direction: FlexDirection::Row,
            gap: 8.0,
            padding: Padding::default(),
            align: Align::Start,
            justify: Justify::Start,
        }),
    ));

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    assert_eq!(output.rp.nodes.len(), 3);

    let RpNode::Text(first) = &output.rp.nodes[0] else {
        panic!("expected first text node");
    };
    let RpNode::Text(second) = &output.rp.nodes[1] else {
        panic!("expected second text node");
    };
    let RpNode::Text(third) = &output.rp.nodes[2] else {
        panic!("expected third text node");
    };

    assert_eq!((first.x, first.y), (4.0, 6.0));
    assert_eq!((second.x, second.y), (4.0, 21.0));
    assert_eq!((third.x, third.y), (45.0, 0.0));
}

#[test]
fn compiles_image_source_as_data() {
    let root = IrNode::<Msg>::Image(Image {
        src: ImageSource::Path(PathBuf::from("logo.png")),
        style: ImageStyle {
            size: Size {
                width: 32.0,
                height: 24.0,
            },
        },
    });

    let output = compile(
        &root,
        Size {
            width: 800.0,
            height: 600.0,
        },
    );

    let RpNode::Image(image) = &output.rp.nodes[0] else {
        panic!("expected image node");
    };

    assert_eq!(image.width, 32.0);
    assert_eq!(image.height, 24.0);
    assert_eq!(image.src, ImageSource::Path(PathBuf::from("logo.png")));
}
