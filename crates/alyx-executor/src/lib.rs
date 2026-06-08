use alyx_plan::{EventPlan, EventType, HandlerTable, RenderingPlan, ResolvedHitArea};
use std::cmp::Reverse;

pub trait RenderingPlanExecutor {
    fn execute(&mut self, plan: &RenderingPlan);
}

pub trait EventPlanExecutor<Msg> {
    fn execute(&mut self, event_plan: &EventPlan, handlers: &HandlerTable<Msg>) -> Vec<Msg>;
}

#[derive(Debug, Default)]
pub struct TraceRenderer {
    pub calls: Vec<String>,
}

#[derive(Debug, Default)]
pub struct MemoryRenderer {
    pub last_render_plan: Option<RenderingPlan>,
}

impl RenderingPlanExecutor for TraceRenderer {
    fn execute(&mut self, plan: &RenderingPlan) {
        for node in &plan.nodes {
            match node {
                alyx_plan::RpNode::Text(text) => self.calls.push(format!(
                    "text:{}:{}@{},{}",
                    text.content, text.element.node_id.0, text.x, text.y
                )),
                alyx_plan::RpNode::Image(image) => self.calls.push(format!(
                    "image:{:?}:{}@{},{}",
                    image.src, image.element.element_id.0, image.x, image.y
                )),
            }
        }
    }
}

impl RenderingPlanExecutor for MemoryRenderer {
    fn execute(&mut self, plan: &RenderingPlan) {
        self.last_render_plan = Some(plan.clone());
    }
}

#[derive(Debug, Default)]
pub struct NullEventExecutor;

impl<Msg> EventPlanExecutor<Msg> for NullEventExecutor
where
    Msg: Clone,
{
    fn execute(&mut self, _event_plan: &EventPlan, _handlers: &HandlerTable<Msg>) -> Vec<Msg> {
        Vec::new()
    }
}

pub fn resolve_handler_targets<Msg>(
    event: ResolvedHitArea,
    handlers: &HandlerTable<Msg>,
) -> Option<Msg>
where
    Msg: Clone,
{
    handlers.get(event.handler_id).cloned()
}

pub fn hit_areas_at(plan: &EventPlan, x: f32, y: f32) -> Vec<&ResolvedHitArea> {
    let mut matched: Vec<&ResolvedHitArea> = plan
        .hit_areas
        .iter()
        .filter(|area| {
            x >= area.rect.x
                && y >= area.rect.y
                && x <= area.rect.x + area.rect.width
                && y <= area.rect.y + area.rect.height
        })
        .collect();

    matched.sort_by_key(|area| Reverse(area.z_index));
    matched
}

pub fn pick_topmost_hit(plan: &EventPlan, x: f32, y: f32) -> Option<&ResolvedHitArea> {
    hit_areas_at(plan, x, y).into_iter().next()
}

pub fn pick_topmost_hit_of_type(
    plan: &EventPlan,
    x: f32,
    y: f32,
    event_type: EventType,
) -> Option<&ResolvedHitArea> {
    hit_areas_at_type(plan, x, y, event_type).into_iter().next()
}

pub fn hit_areas_at_type(
    plan: &EventPlan,
    x: f32,
    y: f32,
    event_type: EventType,
) -> Vec<&ResolvedHitArea> {
    let mut matched: Vec<&ResolvedHitArea> = plan
        .hit_areas
        .iter()
        .filter(|area| {
            area.event_type == event_type
                && x >= area.rect.x
                && y >= area.rect.y
                && x <= area.rect.x + area.rect.width
                && y <= area.rect.y + area.rect.height
        })
        .collect();

    matched.sort_by_key(|area| Reverse(area.z_index));
    matched
}

pub fn dispatch_event<Msg: Clone>(
    plan: &EventPlan,
    handlers: &HandlerTable<Msg>,
    x: f32,
    y: f32,
) -> Vec<Msg> {
    pick_topmost_hit(plan, x, y)
        .and_then(|hit| resolve_handler_targets(hit.clone(), handlers).map(|msg| vec![msg]))
        .unwrap_or_default()
}

pub fn dispatch_event_of_type<Msg: Clone>(
    plan: &EventPlan,
    handlers: &HandlerTable<Msg>,
    x: f32,
    y: f32,
    event_type: EventType,
) -> Vec<Msg> {
    pick_topmost_hit_of_type(plan, x, y, event_type)
        .and_then(|hit| resolve_handler_targets(hit.clone(), handlers).map(|msg| vec![msg]))
        .unwrap_or_default()
}
