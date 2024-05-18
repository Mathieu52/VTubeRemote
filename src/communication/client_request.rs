use rocket::FromForm;
use rocket::serde::{Deserialize};

#[derive(Debug, Clone)]
#[derive(Deserialize)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(PartialEq))]
#[serde(crate = "rocket::serde")]
pub enum ClientRequest {
    TriggerHotKey { id: String },
    SetHotkeyIcon { id: String },
}