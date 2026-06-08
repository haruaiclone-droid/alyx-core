use alyx_ir::{ElementId, NodeId, Rect};

use crate::HandlerId;

#[derive(Clone, Debug, PartialEq)]
pub struct EventPlan {
    pub hit_areas: Vec<ResolvedHitArea>,
    pub focus_order: Vec<ResolvedHitTarget>,
    pub navigation: crate::NavigationPlan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedHitTarget {
    pub node_id: NodeId,
    pub element_id: ElementId,
    pub rect: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedHitArea {
    pub rect: Rect,
    pub handler_id: HandlerId,
    pub event_type: EventType,
    pub node_id: NodeId,
    pub element_id: ElementId,
    pub z_index: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventType {
    Click,
    Hover,
    PointerDown,
    PointerUp,
    PointerMove,
    KeyDown,
    KeyUp,
    Focus,
    Blur,
    Scroll,
    NavigateBack,
    NavigateForward,
    Submit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerEventKind {
    Down,
    Up,
    Move,
}

impl Default for EventPlan {
    fn default() -> Self {
        Self {
            hit_areas: Vec::new(),
            focus_order: Vec::new(),
            navigation: crate::NavigationPlan::empty(),
        }
    }
}
