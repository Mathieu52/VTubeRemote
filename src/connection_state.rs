#[derive(Debug, Clone)]
pub enum ConnectionState {
    Connected,
    Disconnected, // Disconnected normally, we were notified of the disconnection
}
