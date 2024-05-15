enum ConnectionState {
    Connected,
    Disconnected, // Disconnected normally, we were notified of the disconnection
    DisconnectedAbnormally // Disconnected abnormally, we weren't told about the disconnection, we can only observe that it doesn't work anymore
}
