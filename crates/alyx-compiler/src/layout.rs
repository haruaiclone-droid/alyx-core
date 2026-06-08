use alyx_ir::{Container, ElementId, FlexDirection, Layout, NodeId, Size};
use alyx_plan::RpNode;

use crate::compile::compile_to_rp;
use crate::context::CompileContext;

pub(crate) fn compile_container<Msg>(
    node_id: NodeId,
    container: &Container<Msg>,
    x: f32,
    y: f32,
    available: Size,
    context: &mut CompileContext<Msg>,
) -> Vec<RpNode>
where
    Msg: Clone + Send,
{
    let Layout::Flex(layout) = container.layout;
    let _ = node_id;
    let _ = available;

    let mut children = Vec::with_capacity(container.children.len());
    let mut cursor_x = x + layout.padding.left;
    let mut cursor_y = y + layout.padding.top;

    let spacing = layout.gap;

    let total_child_width: f32 = container
        .children
        .iter()
        .map(|child| child.size().width)
        .sum();
    let total_child_height: f32 = container
        .children
        .iter()
        .map(|child| child.size().height)
        .sum();

    let cross_size = match layout.direction {
        FlexDirection::Row => container
            .children
            .iter()
            .map(|child| child.size().height)
            .fold(0.0_f32, f32::max),
        FlexDirection::Column => container
            .children
            .iter()
            .map(|child| child.size().width)
            .fold(0.0_f32, f32::max),
    };

    let justify_offset = match layout.direction {
        FlexDirection::Row => {
            let used =
                total_child_width + spacing * ((container.children.len().saturating_sub(1)) as f32);
            (available.width - used - layout.padding.left - layout.padding.right).max(0.0)
        }
        FlexDirection::Column => {
            let used = total_child_height
                + spacing * ((container.children.len().saturating_sub(1)) as f32);
            (available.height - used - layout.padding.top - layout.padding.bottom).max(0.0)
        }
    };

    match (layout.direction, layout.justify) {
        (FlexDirection::Row, alyx_ir::Justify::Start) => {}
        (FlexDirection::Row, alyx_ir::Justify::Center) => cursor_x += justify_offset / 2.0,
        (FlexDirection::Row, alyx_ir::Justify::End) => cursor_x += justify_offset,
        (FlexDirection::Column, alyx_ir::Justify::Start) => {}
        (FlexDirection::Column, alyx_ir::Justify::Center) => cursor_y += justify_offset / 2.0,
        (FlexDirection::Column, alyx_ir::Justify::End) => cursor_y += justify_offset,
    }

    for (index, child) in container.children.iter().enumerate() {
        if index > 0 {
            match layout.direction {
                FlexDirection::Row => cursor_x += layout.gap,
                FlexDirection::Column => cursor_y += layout.gap,
            }
        }

        let child_size = child.size();
        children.extend(compile_to_rp(
            child, cursor_x, cursor_y, child_size, context,
        ));

        match layout.direction {
            FlexDirection::Row => cursor_x += child_size.width,
            FlexDirection::Column => cursor_y += child_size.height,
        }
    }

    let element_id = ElementId(context.alloc_element_id().0);
    context.add_accessibility_entry(
        node_id,
        element_id,
        alyx_ir::Rect {
            x,
            y,
            width: if layout.direction == FlexDirection::Row {
                total_child_width + layout.padding.left + layout.padding.right + justify_offset
            } else {
                cross_size + layout.padding.left + layout.padding.right
            },
            height: if layout.direction == FlexDirection::Row {
                cross_size + layout.padding.top + layout.padding.bottom
            } else {
                total_child_height + layout.padding.top + layout.padding.bottom + justify_offset
            },
        },
        alyx_ir::AccessibilityMetadata::new(),
    );

    children
}
