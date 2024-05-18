#[derive(Debug, Clone)]
pub enum ClientRequest {
    TriggerHotKey { id: String },
}