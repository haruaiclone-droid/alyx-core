use alyx_ir::{Container, FlexDirection, Layout, Size};
use alyx_plan::RpContainer;

use crate::compile::compile_node;
use crate::context::CompileContext;

pub(crate) fn compile_container<Msg>(
    container: &Container<Msg>,
    x: f32,
    y: f32,
    available: Size,
    context: &mut CompileContext<Msg>,
) -> RpContainer
where
    Msg: Clone + Send,
{
    let size = resolved_size(container.size, available);
    let Layout::Flex(layout) = container.layout;
    let mut children = Vec::with_capacity(container.children.len());

    let mut cursor_x = x + layout.padding.left;
    let mut cursor_y = y + layout.padding.top;

    for child in &container.children {
        let child_size = child.size();
        children.push(compile_node(child, cursor_x, cursor_y, child_size, context));

        match layout.direction {
            FlexDirection::Row => {
                cursor_x += child_size.width + layout.gap;
            }
            FlexDirection::Column => {
                cursor_y += child_size.height + layout.gap;
            }
        }
    }

    RpContainer {
        x,
        y,
        width: size.width,
        height: size.height,
        children,
    }
}

fn resolved_size(size: Size, available: Size) -> Size {
    Size {
        width: if size.width > 0.0 {
            size.width
        } else {
            available.width
        },
        height: if size.height > 0.0 {
            size.height
        } else {
            available.height
        },
    }
}
