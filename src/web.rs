use std::sync::{Arc, RwLock};
use rocket::{Build, get, Rocket, routes, State};
use rocket::fs::{FileServer, relative};
use rocket::response::stream::EventStream;
use rocket_slogger::Slogger;
use slog::Logger;
use slog_syslog::Facility;
use tokio::sync::broadcast::Sender;
use vtubestudio::data::Hotkey;
use crate::change::Change;
use crate::change::Change::Added;
use crate::communication::server_event::ServerEvent;
use crate::hotkey_state::HotkeyState;
use crate::communication::client_request::ClientRequest;
use crate::communication::client_request::ClientRequest::TriggerHotKey;
use crate::state::vtube_state::VTubeState;
use crate::state::vtube_streams::VTubeStreams;

#[get("/<id>")]
async fn trigger_hot_key(streams: &State<VTubeStreams>, id: &str) {
    if let Ok(guard) = streams.requests.read() {
        if let Err(e) = guard.send(TriggerHotKey {id: id.to_string()}) {
            eprintln!("Error sending request: {:?}", e);
        }
    } else {
        println!("Unable to send request");
    }
}


#[get("/events")]
async fn events(state: &State<VTubeState>, streams: &State<VTubeStreams>) -> EventStream![] {
//async fn events(queue: &State<Arc<RwLock<Sender<Change<(Hotkey, Option<HotkeyState>)>>>>>, hotkey_states: &State<Arc<RwLock<Vec<(Hotkey, Option<HotkeyState>)>>>>) -> EventStream![] {
    let mut rx = streams.events.read().unwrap().subscribe();
    // Collect initial hotkey states into owned data
    let initial_events = {
        let hotkey_states_lock = state.hotkey_states.read().unwrap();
        hotkey_states_lock
            .iter()
            .map(|(hotkey, state)| {
                let emote_info = ServerEvent::from(Added((hotkey.clone(), state.clone())));
                rocket::response::stream::Event::json(&emote_info)
            })
            .collect::<Vec<_>>()
    };

    let connection_state = state.connection_state.read().unwrap().clone();
    let connection_event = ServerEvent::from(connection_state);

    EventStream! {
        for event in initial_events {
            yield event;
        }

        yield rocket::response::stream::Event::json(&connection_event);

        loop {
            if let Ok(info) = rx.recv().await {
                let info = ServerEvent::from(info.clone());
                yield rocket::response::stream::Event::json(&info);
            }
        }
    }
}

pub fn rocket() -> Rocket<Build> {
    let rocket = rocket::build().mount("/", routes![events])
        .mount("/trigger", routes![trigger_hot_key])
        .mount("/", FileServer::from(relative!("static")));


    rocket
}