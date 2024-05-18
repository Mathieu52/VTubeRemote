use std::sync::{Arc, RwLock};
use rocket::tokio::sync::broadcast::{Sender};
use crate::communication::client_request::ClientRequest;
use crate::communication::server_event::ServerEvent;

#[derive(Debug, Clone)]
pub struct VTubeStreams {
    pub events: Arc<RwLock<Sender<ServerEvent>>>,
    pub requests: Arc<RwLock<Sender<ClientRequest>>>
}