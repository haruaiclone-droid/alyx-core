use alyx_ir::{AccessibilityMetadata, ElementId, ImageSource, NodeId, Rect, TextStyle};

#[derive(Clone, Debug, PartialEq)]
pub struct RenderingPlan {
    pub nodes: Vec<RpNode>,
    pub accessibility: AccessibilityPlan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessibilityPlan {
    pub entries: Vec<AccessibilityPlanEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessibilityPlanEntry {
    pub node_id: NodeId,
    pub element_id: ElementId,
    pub metadata: AccessibilityMetadata,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderingElement {
    pub node_id: NodeId,
    pub element_id: ElementId,
}

pub type RpTextStyle = TextStyle;

#[derive(Clone, Debug, PartialEq)]
pub enum RpNode {
    Text(RpText),
    Image(RpImage),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RpText {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub content: String,
    pub style: RpTextStyle,
    pub element: RenderingElement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RpImage {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub src: ImageSource,
    pub element: RenderingElement,
}

impl Default for RenderingPlan {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            accessibility: AccessibilityPlan {
                entries: Vec::new(),
            },
        }
    }
}

impl AccessibilityPlan {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: AccessibilityPlanEntry) {
        self.entries.push(entry);
    }
}

impl Default for AccessibilityPlan {
    fn default() -> Self {
        Self::new()
    }
}
