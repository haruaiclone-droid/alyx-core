use alyx_ir::{Container, FlexDirection, Layout, Size};
use alyx_plan::RpNode;

use crate::compile::compile_to_rp;
use crate::context::CompileContext;

pub(crate) fn compile_container<Msg>(
    container: &Container<Msg>,
    x: f32,
    y: f32,
    _available: Size,
    context: &mut CompileContext<Msg>,
) -> Vec<RpNode>
where
    Msg: Clone + Send,
{
    let Layout::Flex(layout) = container.layout;
    let mut children = Vec::with_capacity(container.children.len());

    let mut cursor_x = x + layout.padding.left;
    let mut cursor_y = y + layout.padding.top;

    for child in &container.children {
        let child_size = child.size();
        children.extend(compile_to_rp(
            child, cursor_x, cursor_y, child_size, context,
        ));

        match layout.direction {
            FlexDirection::Row => {
                cursor_x += child_size.width + layout.gap;
            }
            FlexDirection::Column => {
                cursor_y += child_size.height + layout.gap;
            }
        }
    }

    children
}
