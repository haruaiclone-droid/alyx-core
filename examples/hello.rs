use alyx_core::{ir::*, widgets::*};

fn main() {
    let root: alyx_core::ir::IrNode<()> = column::<()>([
        Widget::Text(text("Hello Alyx").size(200.0, 24.0)),
        Widget::Text(text("Complete UI pipeline demo").size(240.0, 18.0)),
    ])
    .into_ir();

    let output = alyx_core::compiler::compile(
        &root,
        Size {
            width: 320.0,
            height: 120.0,
        },
    );
    println!("rendered nodes: {}", output.rp.nodes.len());
}
