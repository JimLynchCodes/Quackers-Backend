use crate::quackers_game::client_msg_handler::client_msg;
use crate::quackers_game::types::defaults::{
    PLAYER_X_DEFAULT_START_POSTION, PLAYER_Y_DEFAULT_START_POSTION,
};
use crate::quackers_game::types::game_state::{ClientConnection, ClientGameData};
use crate::{ClientConnections, ClientsGameData, Cracker};

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

pub async fn client_connection(
    ws: WebSocket,
    client_connections: ClientConnections,
    clients_game_data: ClientsGameData,
    cracker: Cracker,
) {
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

    let new_client_connection = ClientConnection {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    client_connections
        .lock()
        .await
        .insert(uuid.clone(), new_client_connection);

    let new_client_game_data = ClientGameData {
        // sender: Some(client_sender),
        client_id: uuid.clone(),
        friendly_name: "[NO_NAME]".to_string(),
        color: "red".to_string(),
        quack_pitch: 1.0,
        x_pos: PLAYER_X_DEFAULT_START_POSTION,
        y_pos: PLAYER_Y_DEFAULT_START_POSTION,
        radius: 20,
        cracker_count: 0,
    };

    clients_game_data
        .lock()
        .await
        .insert(uuid.clone(), new_client_game_data);

    // while let Some(result) = client_ws_rcv.next().await {
    //     let msg = match result {
    //         Ok(msg) => msg,
    //         Err(e) => {
    //             println!("error receiving message for id {}): {}", uuid.clone(), e);
    //             break;
    //         }
    //     };
    //     client_msg(
    //         &uuid,
    //         msg,
    //         &client_connections,
    //         &clients_game_data,
    //         &cracker,
    //     )
    //     .await;
    // }

    // Use a `loop` with an error handling block to ensure cleanup happens
    let disconnect_reason = loop {
        match client_ws_rcv.next().await {
            Some(Ok(msg)) => {
                // Process the message
                let result = client_msg(
                    &uuid,
                    msg,
                    &client_connections,
                    &clients_game_data,
                    &cracker,
                )
                .await;
                println!("processed message 👍")
            }
            Some(Err(e)) => {
                // Handle WebSocket errors (e.g., network issues)
                println!("error receiving message for id {}: {}", uuid.clone(), e);
                break Some(format!("WebSocket error: {}", e)); // Capture error reason
            }
            None => {
                // The client disconnected normally (e.g., closed their WebSocket)
                println!("Client {} disconnected gracefully", uuid);
                break None; // Graceful disconnection, no error
            }
        }
    };

    client_connections.lock().await.remove(&uuid);
    clients_game_data.lock().await.remove(&uuid);

    if let Some(reason) = disconnect_reason {
        println!("{} disconnected with reason: {}", uuid, reason);
    } else {
        println!("{} disconnected gracefully", uuid);
    }
}
