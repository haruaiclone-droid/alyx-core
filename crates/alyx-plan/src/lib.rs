mod event;
mod handler;
mod navigation;
mod rendering;

pub use event::{EventPlan, EventType, PointerEventKind, ResolvedHitArea, ResolvedHitTarget};
pub use handler::{HandlerId, HandlerTable};
pub use navigation::{NavigationAction, NavigationPlan};
pub use rendering::{
    AccessibilityPlan, AccessibilityPlanEntry, RenderingElement, RenderingPlan, RpImage, RpNode,
    RpText, RpTextStyle,
};

#[derive(Clone, Debug, PartialEq)]
pub struct CompilerOutput<Msg> {
    pub rp: RenderingPlan,
    pub ep: EventPlan,
    pub handlers: HandlerTable<Msg>,
}

#[cfg(test)]
mod tests {
    use super::{EventPlan, EventType, NavigationAction, NavigationPlan};

    #[test]
    fn event_plan_default_is_empty_payload() {
        let plan = EventPlan::default();
        assert!(plan.hit_areas.is_empty());
        assert!(plan.focus_order.is_empty());
        assert_eq!(plan.navigation, NavigationPlan::empty());
    }

    #[test]
    fn navigation_actions_support_back_and_forward() {
        let navigation = NavigationPlan {
            actions: vec![
                NavigationAction::NavigateBack,
                NavigationAction::NavigateForward,
            ],
        };

        assert!(navigation.actions.contains(&NavigationAction::NavigateBack));
        assert!(
            navigation
                .actions
                .contains(&NavigationAction::NavigateForward)
        );
    }

    #[test]
    fn event_types_cover_navigation_and_input() {
        let all = [
            EventType::Click,
            EventType::Hover,
            EventType::PointerDown,
            EventType::PointerUp,
            EventType::PointerMove,
            EventType::KeyDown,
            EventType::KeyUp,
            EventType::Focus,
            EventType::Blur,
            EventType::Scroll,
            EventType::NavigateBack,
            EventType::NavigateForward,
            EventType::Submit,
        ];
        assert_eq!(all.len(), 13);
    }
}
