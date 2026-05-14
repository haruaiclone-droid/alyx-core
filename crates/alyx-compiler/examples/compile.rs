use alyx_compiler::compile;
use alyx_ir::{Color, Font, HitArea, IrNode, Rect, Size, Text, TextStyle};

#[derive(Clone, Debug)]
pub enum Message {
    OnClick,
}

fn main() {
    let node: IrNode<Message> = IrNode::HitArea(HitArea {
        rect: Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 200.0,
        },
        on_click: Some(Message::OnClick),
        on_hover: None,
        child: Box::new(IrNode::Text(Text {
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
        })),
    });

    let result = compile(
        &node,
        Size {
            width: 800.0,
            height: 1200.0,
        },
    );

    println!("{result:?}")
}
