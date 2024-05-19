use rocket::FromForm;
use rocket::serde::{Deserialize};

#[derive(Debug, Clone)]
#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(crate = "rocket::serde")]
pub enum ClientRequest {
    TriggerHotKey { id: String },
    UpdateIcon { id: String },
}