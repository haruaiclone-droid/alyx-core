use alyx_compiler::compile;
use alyx_ir::{
    Align, Color, FlexLayout, Font, HitArea, IrNode, Justify, Layout, Padding, Size, Text,
    TextStyle,
};

#[derive(Clone, Debug)]
pub enum Message {
    OnClick,
}

fn main() {
    let node: IrNode<Message> = IrNode::HitArea(HitArea::new(
        Layout::Flex(FlexLayout {
            direction: alyx_ir::FlexDirection::Column,
            gap: 0.0,
            padding: Padding {
                left: 8.0,
                right: 8.0,
                top: 4.0,
                bottom: 4.0,
            },
            align: Align::Start,
            justify: Justify::Start,
        }),
        IrNode::Text(Text {
            content: "hello".to_string(),
            style: TextStyle {
                font: Font {
                    family: "aa".to_string(),
                },
                size: 4.0,
                color: Color {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0,
                    a: 0.0,
                },
            },
            size: Size {
                width: 100.0,
                height: 50.0,
            },
        }),
        Some(Message::OnClick),
        None,
    ));

    let result = compile(
        &node,
        Size {
            width: 800.0,
            height: 1200.0,
        },
    );

    println!("{result:?}")
}
