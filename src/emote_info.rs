use rocket::serde::{Deserialize, Serialize};
use rocket::{FromForm};
use vtubestudio::data::Hotkey;
use crate::change::Change;
use crate::hotkey_state::HotkeyState;
use crate::hotkey_state::HotkeyState::{Active, Inactive};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(crate = "rocket::serde")]
pub struct EmoteInfo {
    type_: String,
    #[field(validate = len(..30))]
    id: String,
    #[field(validate = len(..20))]
    name: String,
    active: Option<bool>,
    time_left: Option<u128>,
}

impl EmoteInfo {
    pub fn from(info: Change<(Hotkey, Option<HotkeyState>)>) -> Self {
        let id;
        let name;
        let mut active= None;
        let mut time_left = None;

        let type_ = match info {
            Change::Added(_) => "added".to_string(),
            Change::Changed(_) => "changed".to_string(),
            Change::Removed(_) => "removed".to_string(),
        };

        match info {
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

        Self {type_, id, name, active, time_left }
    }
}
