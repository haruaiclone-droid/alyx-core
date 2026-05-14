use alyx_ir::{IrNode, Rect, Size};
use alyx_plan::{EventType, RenderingPlan, ResolvedHitArea, RpImage, RpNode, RpText};

use crate::context::CompileContext;
use crate::layout::compile_container;
use crate::CompilerOutput;

pub fn compile<Msg>(root: &IrNode<Msg>, viewport: Size) -> CompilerOutput<Msg>
where
    Msg: Clone + Send,
{
    let mut context = CompileContext::new();
    let node = compile_node(root, 0.0, 0.0, viewport, &mut context);

    CompilerOutput {
        rp: RenderingPlan { nodes: vec![node] },
        ep: context.ep,
        handlers: context.handlers,
    }
}

pub(crate) fn compile_node<Msg>(
    node: &IrNode<Msg>,
    x: f32,
    y: f32,
    available: Size,
    context: &mut CompileContext<Msg>,
) -> RpNode
where
    Msg: Clone + Send,
{
    match node {
        IrNode::Container(container) => {
            RpNode::Container(compile_container(container, x, y, available, context))
        }
        IrNode::Text(text) => RpNode::Text(RpText {
            x,
            y,
            width: text.size.width,
            height: text.size.height,
            content: text.content.clone(),
            style: text.style.clone(),
        }),
        IrNode::Image(image) => RpNode::Image(RpImage {
            x,
            y,
            width: image.style.size.width,
            height: image.style.size.height,
            src: image.src.clone(),
        }),
        IrNode::HitArea(hit_area) => {
            if let Some(msg) = hit_area.on_click.clone() {
                let handler_id = context.handlers.insert(msg);
                context.ep.hit_areas.push(ResolvedHitArea {
                    rect: resolve_rect(hit_area.rect, x, y),
                    handler_id,
                    event_type: EventType::Click,
                });
            }

            if let Some(msg) = hit_area.on_hover.clone() {
                let handler_id = context.handlers.insert(msg);
                context.ep.hit_areas.push(ResolvedHitArea {
                    rect: resolve_rect(hit_area.rect, x, y),
                    handler_id,
                    event_type: EventType::Hover,
                });
            }

            compile_node(&hit_area.child, x, y, available, context)
        }
    }
}

fn resolve_rect(rect: Rect, parent_x: f32, parent_y: f32) -> Rect {
    Rect {
        x: parent_x + rect.x,
        y: parent_y + rect.y,
        width: rect.width,
        height: rect.height,
    }
}
