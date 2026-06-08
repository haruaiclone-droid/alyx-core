#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NavigationAction {
    NavigateBack,
    NavigateForward,
    NavigateTo(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavigationPlan {
    pub actions: Vec<NavigationAction>,
}

impl NavigationPlan {
    pub fn empty() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}
