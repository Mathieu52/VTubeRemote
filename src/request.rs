#[derive(Debug, Clone)]
pub enum VTubeRequest {
    TriggerHotKey { id: String },
}