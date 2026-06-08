#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Generic,
    Button,
    Checkbox,
    Link,
    Text,
    Image,
    Pane,
    InputText,
    List,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessibilityMetadata {
    pub role: Role,
    pub label: Option<String>,
    pub description: Option<String>,
    pub disabled: bool,
    pub focusable: bool,
    pub tab_order: Option<u32>,
}

impl AccessibilityMetadata {
    pub fn new() -> Self {
        Self {
            role: Role::Generic,
            label: None,
            description: None,
            disabled: false,
            focusable: false,
            tab_order: None,
        }
    }

    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = focusable;
        self
    }

    pub fn tab_order(mut self, tab_order: u32) -> Self {
        self.tab_order = Some(tab_order);
        self
    }
}

impl Default for AccessibilityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accessibility {
    Unknown,
    Readonly,
    ReadWrite,
}
