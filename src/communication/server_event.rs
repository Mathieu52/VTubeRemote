use rocket::serde::Serialize;
use vtubestudio::data::Hotkey;
use crate::change::Change;
use crate::connection_state::ConnectionState;
use crate::hotkey_state::HotkeyState;
use crate::hotkey_state::HotkeyState::{Active, Inactive};

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(PartialEq))]
#[serde(crate = "rocket::serde")]
pub enum ServerEvent {
    HotkeyChange {
        type_: String,
        id: String,
        name: String,
        active: Option<bool>,
        time_left: Option<u128>
    },

    IconUpdated {
        id: String
    },

    ConnectionStatus {
        status: String
    },
}

impl From<Change<(Hotkey, Option<HotkeyState>)>> for ServerEvent {
    fn from(change: Change<(Hotkey, Option<HotkeyState>)>) -> Self {
        let id;
        let name;
        let mut active = None;
        let mut time_left = None;

        let type_ = match change {
            Change::Added(_) => "added".to_string(),
            Change::Changed(_) => "changed".to_string(),
            Change::Removed(_) => "removed".to_string(),
        };

        match change {
            Change::Added(info) | Change::Changed(info) | Change::Removed(info) => {
                id = info.0.hotkey_id;
                name = info.0.name;

                if let Some(ref state) = info.1 {
                    active = match state {
                        Active(_) => Some(true),
                        Inactive => Some(false),
                    };

                    if let Some(Active(duration)) = info.1.to_owned() {
                        time_left = duration.map(|duration| duration.as_nanos());
                    }
                }
            }
        }

        ServerEvent::HotkeyChange {type_, id, name, active, time_left }
    }
}

impl From<ConnectionState> for ServerEvent {
    fn from(connection_state: ConnectionState) -> Self {
        ServerEvent::ConnectionStatus {
            status: match connection_state {
                ConnectionState::Connected => "connected".to_string(),
                ConnectionState::Disconnected => "disconnected".to_string()
            }
        }
    }
}