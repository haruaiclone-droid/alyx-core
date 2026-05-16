use alyx_ir::{IrNode, Rect, Size};
use alyx_plan::{EventType, RenderingPlan, ResolvedHitArea, RpImage, RpNode, RpText};

use crate::CompilerOutput;
use crate::context::CompileContext;
use crate::layout::compile_container;

pub fn compile<Msg>(root: &IrNode<Msg>, viewport: Size) -> CompilerOutput<Msg>
where
    Msg: Clone + Send,
{
    let mut context = CompileContext::new();
    let nodes = compile_to_rp(root, 0.0, 0.0, viewport, &mut context);

    CompilerOutput {
        rp: RenderingPlan { nodes },
        ep: context.ep,
        handlers: context.handlers,
    }
}

pub(crate) fn compile_to_rp<Msg>(
    node: &IrNode<Msg>,
    x: f32,
    y: f32,
    available: Size,
    context: &mut CompileContext<Msg>,
) -> Vec<RpNode>
where
    Msg: Clone + Send,
{
    let mut rp_nodes = Vec::new();
    match node {
        IrNode::Container(container) => {
            rp_nodes.extend(compile_container(container, x, y, available, context));
        }
        IrNode::Text(text) => rp_nodes.push(RpNode::Text(RpText {
            x,
            y,
            width: text.size.width,
            height: text.size.height,
            content: text.content.clone(),
            style: text.style.clone(),
        })),
        IrNode::Image(image) => rp_nodes.push(RpNode::Image(RpImage {
            x,
            y,
            width: image.style.size.width,
            height: image.style.size.height,
            src: image.src.clone(),
        })),
        IrNode::HitArea(hit_area) => {
            let size = hit_area.size();
            let rect = Rect {
                x,
                y,
                width: size.width,
                height: size.height,
            };

            if let Some(msg) = hit_area.on_click.clone() {
                let handler_id = context.handlers.insert(msg);
                context.ep.hit_areas.push(ResolvedHitArea {
                    rect,
                    handler_id,
                    event_type: EventType::Click,
                });
            }

            if let Some(msg) = hit_area.on_hover.clone() {
                let handler_id = context.handlers.insert(msg);
                context.ep.hit_areas.push(ResolvedHitArea {
                    rect,
                    handler_id,
                    event_type: EventType::Hover,
                });
            }

            let (child_x, child_y) = hit_area.child_origin(x, y);
            rp_nodes.extend(compile_to_rp(
                &hit_area.child,
                child_x,
                child_y,
                available,
                context,
            ));
        }
        IrNode::Pane(_) => {}
    }

    rp_nodes
}
