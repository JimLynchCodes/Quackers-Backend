use crate::quackers_game_logic::client_msg_handler::client_msg;
use crate::quackers_game_logic::types::defaults::{PLAYER_X_DEFAULT_START_POSTION, PLAYER_Y_DEFAULT_START_POSTION};
use crate::quackers_game_logic::types::game_state::Client;
use crate::{Clients, Cracker};

use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use std::convert::TryInto;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;

pub async fn client_connection(ws: WebSocket, clients: Clients, cracker: Cracker) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),

        friendly_name: "[NO_NAME]".to_string(),
        color: "red".to_string(),
        quack_pitch: 1.0,
        x_pos: PLAYER_X_DEFAULT_START_POSTION,
        y_pos: PLAYER_Y_DEFAULT_START_POSTION,
        radius: 20,
        cracker_count: 0,
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients, &cracker).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}
