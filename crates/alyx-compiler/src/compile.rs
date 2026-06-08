use alyx_ir::{AccessibilityMetadata, ElementId, IrNode, NodeId, Rect, Size};
use alyx_plan::{
    AccessibilityPlan, CompilerOutput, EventType, RenderingElement, RenderingPlan, ResolvedHitArea,
    RpImage, RpNode, RpText, RpTextStyle,
};

use crate::context::CompileContext;
use crate::layout::compile_container;

pub fn compile<Msg>(root: &IrNode<Msg>, viewport: Size) -> CompilerOutput<Msg>
where
    Msg: Clone,
{
    let mut context = CompileContext::new();
    let nodes = compile_to_rp(root, 0.0, 0.0, viewport, &mut context);

    CompilerOutput {
        rp: RenderingPlan {
            nodes,
            accessibility: AccessibilityPlan {
                entries: context.accessibility,
            },
        },
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
    Msg: Clone,
{
    let node_id = context.alloc_node_id();
    let mut rp_nodes = Vec::new();
    match node {
        IrNode::Container(container) => {
            rp_nodes.extend(compile_container(
                node_id, container, x, y, available, context,
            ));
        }
        IrNode::Text(text) => {
            let element = RenderingElement {
                node_id,
                element_id: context.alloc_element_id(),
            };
            rp_nodes.push(RpNode::Text(RpText {
                x,
                y,
                width: text.size.width,
                height: text.size.height,
                content: text.content.clone(),
                style: RpTextStyle {
                    font: text.style.font.clone(),
                    size: text.style.size,
                    color: text.style.color,
                },
                element,
            }));
            context.add_accessibility_entry(
                node_id,
                element.element_id,
                Rect {
                    x,
                    y,
                    width: text.size.width,
                    height: text.size.height,
                },
                AccessibilityMetadata::new().role(alyx_ir::Role::Text),
            );
        }
        IrNode::Image(image) => {
            let element = RenderingElement {
                node_id,
                element_id: context.alloc_element_id(),
            };
            rp_nodes.push(RpNode::Image(RpImage {
                x,
                y,
                width: image.style.size.width,
                height: image.style.size.height,
                src: image.src.clone(),
                element,
            }));
            context.add_accessibility_entry(
                node_id,
                element.element_id,
                Rect {
                    x,
                    y,
                    width: image.style.size.width,
                    height: image.style.size.height,
                },
                AccessibilityMetadata::new().role(alyx_ir::Role::Image),
            );
        }
        IrNode::HitArea(hit_area) => {
            let element = RenderingElement {
                node_id,
                element_id: context.alloc_element_id(),
            };
            let size = hit_area.size();
            let rect = Rect {
                x,
                y,
                width: size.width,
                height: size.height,
            };

            let push_area = |context: &mut CompileContext<Msg>,
                             handler: Msg,
                             event_type: EventType,
                             rect: Rect,
                             node_id: NodeId,
                             element_id: ElementId| {
                let handler_id = context.handlers.insert(handler);
                context.ep.hit_areas.push(ResolvedHitArea {
                    rect,
                    handler_id,
                    event_type,
                    node_id,
                    element_id,
                    z_index: 0,
                });
            };

            if !hit_area.disabled {
                if let Some(msg) = hit_area.on_click.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::Click,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_hover.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::Hover,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_key_down.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::KeyDown,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_key_up.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::KeyUp,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_pointer_down.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::PointerDown,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_pointer_up.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::PointerUp,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_pointer_move.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::PointerMove,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_focus.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::Focus,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_blur.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::Blur,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
                if let Some(msg) = hit_area.on_submit.clone() {
                    push_area(
                        context,
                        msg,
                        EventType::Submit,
                        rect,
                        node_id,
                        element.element_id,
                    );
                }
            }

            if let Some(navigate_to) = hit_area.navigate_to.clone() {
                context
                    .ep
                    .navigation
                    .actions
                    .push(alyx_plan::NavigationAction::NavigateTo(navigate_to));
            }

            let (child_x, child_y) = hit_area.child_origin(x, y);
            rp_nodes.extend(compile_to_rp(
                &hit_area.child,
                child_x,
                child_y,
                available,
                context,
            ));
            context.add_accessibility_entry(node_id, element.element_id, rect, {
                let mut metadata = hit_area
                    .accessibility
                    .clone()
                    .unwrap_or_else(AccessibilityMetadata::new);
                metadata.disabled = hit_area.disabled;
                metadata
            });
        }
        IrNode::Pane(_) => {}
    }

    rp_nodes
}
