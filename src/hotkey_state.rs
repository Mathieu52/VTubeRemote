use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum HotkeyState {
    Active(Option<Duration>),
    Inactive,
}