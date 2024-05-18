use std::sync::{Arc, RwLock};
use rocket::{Build, get, post, Rocket, routes, State};
use rocket::form::FromForm;
use rocket::fs::{FileServer, relative};
use rocket::response::stream::EventStream;
use rocket::serde::json::Json;
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

#[post("/", data = "<request>")]
async fn requests(streams: &State<VTubeStreams>, request: Json<ClientRequest>) {
    if let Ok(guard) = streams.requests.read() {
        if let Err(e) = guard.send(request.0) {
            eprintln!("Error sending request: {:?}", e);
        }
    } else {
        println!("Unable to send request");
    }
}


#[get("/events")]
async fn events(state: &State<VTubeState>, streams: &State<VTubeStreams>) -> EventStream![] {
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
        .mount("/request", routes![requests])
        .mount("/", FileServer::from(relative!("static")));


    rocket
}