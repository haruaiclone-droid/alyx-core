use alyx_ir::{AccessibilityMetadata, ElementId, NodeId, Rect};
use alyx_plan::{AccessibilityPlanEntry, EventPlan, HandlerTable, ResolvedHitTarget};

pub(crate) struct CompileContext<Msg> {
    pub ep: EventPlan,
    pub handlers: HandlerTable<Msg>,
    pub accessibility: Vec<AccessibilityPlanEntry>,
    next_node_id: u64,
    next_element_id: u64,
}

impl<Msg> CompileContext<Msg> {
    pub fn new() -> Self {
        Self {
            ep: EventPlan::default(),
            handlers: HandlerTable::new(),
            accessibility: Vec::new(),
            next_node_id: 1,
            next_element_id: 1,
        }
    }

    pub fn alloc_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        NodeId(id)
    }

    pub fn alloc_element_id(&mut self) -> ElementId {
        let id = self.next_element_id;
        self.next_element_id += 1;
        ElementId(id)
    }

    pub fn add_accessibility_entry(
        &mut self,
        node_id: NodeId,
        element_id: ElementId,
        rect: Rect,
        metadata: AccessibilityMetadata,
    ) {
        self.ep.focus_order.push(ResolvedHitTarget {
            node_id,
            element_id,
            rect,
        });

        self.accessibility.push(AccessibilityPlanEntry {
            node_id,
            element_id,
            metadata,
            rect,
        });
    }
}
