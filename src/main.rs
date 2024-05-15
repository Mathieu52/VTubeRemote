#![feature(iter_collect_into)]
#![feature(async_closure)]
#![feature(async_fn_traits)]
#![feature(unboxed_closures)]
#![feature(format_args_nl)]
extern crate core;

mod hotkey_state;
mod benchmark;
mod web_example;
mod change;
mod emote_info;
mod request;
mod web;
mod connection;

use rocket::tokio::sync::broadcast::{channel, Sender};
use std::sync::{Arc, RwLock};
use std::time::{Duration};
use rocket::form::validate::Contains;
use rocket::fairing::AdHoc;
use tokio::{time};
use vtubestudio::data::{EventSubscriptionRequest, ExpressionStateRequest, Hotkey, HotkeysInCurrentModelRequest, HotkeyTriggerRequest, ItemListRequest, TestEventConfig};
use vtubestudio::{Client, ClientEvent, ClientEventStream, Error};
use vtubestudio::data::HotkeyAction::{ToggleExpression, ToggleItemScene};
use crate::hotkey_state::HotkeyState;
use crate::hotkey_state::HotkeyState::{Active, Inactive};
use crate::change::{categorize_changes, Change};
use crate::request::VTubeRequest;
use crate::request::VTubeRequest::TriggerHotKey;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = web::rocket().attach(start_vtube()).launch().await;

    Ok(())
}

fn start_vtube() -> AdHoc {
    AdHoc::on_ignite("Managed Queue", |rocket| async {
        let change_channel = channel::<Change<(Hotkey, Option<HotkeyState>)>>(1024);
        let request_channel = channel::<VTubeRequest>(1024);

        let safe_change_channel = Arc::new(RwLock::new(change_channel.0));
        let safe_hotkey_states = Arc::new(RwLock::new(Vec::new()));
        let safe_request_channel = Arc::new(RwLock::new(request_channel.0));

        tokio::spawn(vtube_main(safe_change_channel.clone(), safe_hotkey_states.clone(), safe_request_channel.clone()));
        rocket.manage(safe_change_channel.clone()).manage(safe_hotkey_states.clone()).manage(safe_request_channel.clone())
    })
}

fn build_client() -> (Client, ClientEventStream) {
    let stored_token = Some("b28b671daf1468c194dc9496e47264040f19f7212c2eb4b40a691b3931bbeec1".to_string());

    Client::builder()
        .auth_token(stored_token)
        .authentication("VTube Remote", "Xzera", None)
        .retry_on_disconnect(true)
        .build_tungstenite()
}
async fn vtube_main(channel: Arc<RwLock<Sender<Change<(Hotkey, Option<HotkeyState>)>>>>, hotkey_states: Arc<RwLock<Vec<(Hotkey, Option<HotkeyState>)>>>, request_channel: Arc<RwLock<Sender<VTubeRequest>>>) -> Result<(), Error> {
    let (mut client, mut events) = build_client();

    let req = EventSubscriptionRequest::subscribe(&TestEventConfig { test_message_for_event: "Hello from vtubestudio-rs!".to_string() })?;

    let mut interval = time::interval(Duration::from_millis(100));

    let mut rx_request = request_channel.read().unwrap().subscribe();

    loop {
        tokio::select! {
            Some(client_event) = events.next() => {
                match client_event {
                    // We receive a `Disconnected` client event whenever we are disconnected, including on
                    // startup. This can be used as a cue to refresh any event subscriptions.
                    ClientEvent::Disconnected => {
                        println!("Connecting...");

                        // Try to subscribe to test events, retrying on failure. Note that the client
                        // attempts to reconnect automatically when sending a request.
                        while let Err(e) = client.send(&req).await {
                            eprintln!("Failed to subscribe to test events: {e}");
                            eprintln!("Retrying in 2s...");
                            tokio::time::sleep(Duration::from_secs(2)).await;
                        }
                    }

                    ClientEvent::Api(event) => {
                        println!("Received API event: {:?}", event);
                    }

                    ClientEvent::Error(e) => {
                        println!("Received error: {:?}", e);
                    }

                    other => {
                        println!("Received event: {:?}", other);
                    }
                }
            }
            Ok(request) = rx_request.recv() => { process_request(&mut client, request).await?; }
            _ = interval.tick() => {
                if let Err(err) = regular_process(channel.clone(), &mut client, hotkey_states.clone()).await {
                    eprintln!("Error occurred in regular process: {}", err);
                }
            }
        }
    }
}

async fn process_request(client: &mut Client, request: VTubeRequest) -> Result<(), Error> {
    match request {
        TriggerHotKey{ id } => {
            client.send(&HotkeyTriggerRequest {hotkey_id: id, item_instance_id: None}).await?;
        }
    };

    Ok(())
}
async fn regular_process(channel: Arc<RwLock<Sender<Change<(Hotkey, Option<HotkeyState>)>>>>, client: &mut Client, hotkey_states: Arc<RwLock<Vec<(Hotkey, Option<HotkeyState>)>>>) -> Result<(), Error> {
    let new_hotkey_states = get_hotkey_state(client, None, None, None).await?;

    let changes = categorize_changes(&hotkey_states.read().unwrap(), &new_hotkey_states, |(h, _)| h.hotkey_id.clone());

    let channel = channel.read().unwrap();

    if !changes.is_empty() {
        println!("{} changes found!", changes.len());
    }

    for change in changes {
        let _res = channel.send(change);
    }

    let mut hotkey_states_access = hotkey_states.write().unwrap();
    *hotkey_states_access = new_hotkey_states.iter().cloned().collect();

    Ok(())
}

async fn get_hotkey_state(client: &mut Client, model_id: Option<String>, live2d_item_file_name: Option<String>, hotkey_ids: Option<Vec<String>>) -> Result<Vec<(Hotkey, Option<HotkeyState>)>, Error> {
    let available_hotkeys = client
        .send(&HotkeysInCurrentModelRequest {
            model_id,
            live2d_item_file_name,
        })
        .await?
        .available_hotkeys;

    let item_instances_in_scene = client
        .send(&ItemListRequest {
            include_available_spots: false,
            include_item_instances_in_scene: true,
            include_available_item_files: false,
            only_items_with_file_name: None,
            only_items_with_instance_id: None,
        })
        .await?
        .item_instances_in_scene;

    let expressions = client
        .send(&ExpressionStateRequest {
            details: false,
            expression_file: None,
        })
        .await?
        .expressions;

    // Prepare to collect hotkey states with preallocated capacity
    let mut hotkeys_state: Vec<(Hotkey, Option<HotkeyState>)> = Vec::with_capacity(
        hotkey_ids.as_ref().map_or_else(|| available_hotkeys.len(), |ids| ids.len())
    );

    for hotkey in &available_hotkeys {
        // Skip hotkey if not in hotkey_ids (if hotkey_ids is Some)
        if let Some(ref hotkey_ids) = hotkey_ids {
            if hotkey_ids.contains(&hotkey.hotkey_id) {
                continue;
            }
        }

        let hotkey_state = if hotkey.type_ == ToggleItemScene {
            let is_active = item_instances_in_scene.iter().any(|i| i.scene_name == hotkey.file);
            if is_active {
                Some(Active(None))
            } else {
                Some(Inactive)
            }
        } else if hotkey.type_ == ToggleExpression {
            expressions
                .iter()
                .find(|i| i.active && i.file == hotkey.file)
                .map(|expression| {
                    if expression.auto_deactivate_after_seconds {
                        Some(Duration::from_secs_f64(expression.seconds_remaining))
                    } else {
                        None
                    }
                })
                .map(|time_remaining| Active(time_remaining))
                .or(Some(Inactive))
        } else {
            None
        };

        if let Some(state) = hotkey_state {
            hotkeys_state.push((hotkey.clone(), Some(state)));
        } else {
            hotkeys_state.push((hotkey.clone(), None));
        }
    }

    Ok(hotkeys_state)
}