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
use crate::emote_info::EmoteInfo;
use crate::hotkey_state::HotkeyState;
use crate::request::VTubeRequest;
use crate::request::VTubeRequest::TriggerHotKey;

#[get("/<id>")]
async fn trigger_hot_key(request_channel: &State<Arc<RwLock<Sender<VTubeRequest>>>>, id: &str) {
    if let Ok(guard) = request_channel.read() {
        if let Err(e) = guard.send(TriggerHotKey {id: id.to_string()}) {
            eprintln!("Error sending request: {:?}", e);
        }
    } else {
        println!("Unable to send request");
    }
}


#[get("/events")]
async fn events(queue: &State<Arc<RwLock<Sender<Change<(Hotkey, Option<HotkeyState>)>>>>>, hotkey_states: &State<Arc<RwLock<Vec<(Hotkey, Option<HotkeyState>)>>>>) -> EventStream![] {
    let mut rx = queue.read().unwrap().subscribe();
    // Collect initial hotkey states into owned data
    let initial_events = {
        let hotkey_states_lock = hotkey_states.read().unwrap();
        hotkey_states_lock
            .iter()
            .map(|(hotkey, state)| {
                let emote_info = EmoteInfo::from(Added((hotkey.clone(), state.clone())));
                rocket::response::stream::Event::json(&emote_info)
            })
            .collect::<Vec<_>>()
    };

    EventStream! {
        for event in initial_events {
            yield event;
        }

        loop {
            if let Ok(info) = rx.recv().await {
                let info = EmoteInfo::from(info.clone());
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