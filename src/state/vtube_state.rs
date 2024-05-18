use std::sync::{Arc, RwLock};
use vtubestudio::data::Hotkey;
use crate::connection_state::ConnectionState;
use crate::hotkey_state::HotkeyState;

#[derive(Debug, Clone)]
pub struct VTubeState {
    pub hotkey_states: Arc<RwLock<Vec<(Hotkey, Option<HotkeyState>)>>>,
    pub connection_state: Arc<RwLock<ConnectionState>>,
}