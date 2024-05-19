use std::fmt::format;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};
use rocket::{Build, Data, get, post, Rocket, routes, State};
use rocket::form::FromForm;
use rocket::fs::{FileServer, relative};
use rocket::http::ContentType;
use rocket::response::stream::EventStream;
use rocket::serde::json::Json;
use rocket_multipart_form_data::{multer, MultipartFormData, MultipartFormDataError, MultipartFormDataField, MultipartFormDataOptions};
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
use crate::communication::client_request::ClientRequest::{TriggerHotKey, UpdateIcon};
use crate::state::vtube_state::VTubeState;
use crate::state::vtube_streams::VTubeStreams;

const RESOURCE_PATH: &str = "static/resources";

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

#[post("/", data = "<data>")]
async fn upload(content_type: &ContentType, data: Data<'_>, streams: &State<VTubeStreams>) -> Result<(), &'static str> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(
        vec! [
            MultipartFormDataField::file("image").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap(),
            MultipartFormDataField::text("id"),
        ]
    );

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).await.unwrap();

    let photo = multipart_form_data.files.get("image");
    let id = multipart_form_data.texts.remove("id");

    if let Some(file_fields) = photo {
        if let Some(mut text_fields) = id {
            let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

            let _content_type = &file_field.content_type;
            let _file_name = &file_field.file_name;
            let _path = &file_field.path;

            let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.
            let id = text_field.text;

            if let Some(file_name) = _file_name {
                println!("File: {}", file_name);
            }

            println!("File path: {:?}", _path.as_os_str());

            let path = Path::join(RESOURCE_PATH.as_ref(), format!("{}", id));

            println!("{:?}", path.as_os_str());

            // Canonicalize the path to ensure it is safe and absolute
            //if let Ok(canonical_path) = path.canonicalize() {
                //if canonical_path.starts_with(RESOURCE_PATH) {
                    fs::copy(_path, &path).unwrap();

                    if let Ok(guard) = streams.requests.read() {
                        if let Err(e) = guard.send(UpdateIcon {id}) {
                            eprintln!("Error sending request: {:?}", e);
                        } else {
                            println!("Sent update icon request!");
                        }
                    } else {
                        println!("Unable to send request");
                    }
                //}
            //}
        }
    }

    Ok(())
}

pub fn rocket() -> Rocket<Build> {
    let rocket = rocket::build().mount("/", routes![events])
        .mount("/request", routes![requests])
        .mount("/upload", routes![upload])
        .mount("/", FileServer::from(relative!("static")));


    rocket
}