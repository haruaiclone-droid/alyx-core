use alyx_ir::Rect;

use crate::HandlerId;

#[derive(Clone, Debug, PartialEq)]
pub struct EventPlan {
    pub hit_areas: Vec<ResolvedHitArea>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedHitArea {
    pub rect: Rect,
    pub handler_id: HandlerId,
    pub event_type: EventType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Click,
    Hover,
}
