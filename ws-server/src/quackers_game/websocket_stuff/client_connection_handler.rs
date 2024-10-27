use crate::quackers_game::game::game_constants::{PLAYER_RADIUS, PLAYER_X_DEFAULT_START_POSTION, PLAYER_Y_DEFAULT_START_POSTION};
use crate::quackers_game::game::game_state::{ClientConnection, ClientGameData};
use crate::quackers_game::messages::join::receive_submit_name_request::{
    build_leaderboard_update_msg, recalculate_leaderboard_positions,
};

use crate::quackers_game::types::player_join_msg::DuckDirection;
use crate::quackers_game::websocket_stuff::client_msg_handler::client_msg_handler;
use crate::quackers_game::websocket_stuff::disconnect::user_disconnected_msg::build_user_disconnected_msg;
use crate::{ClientConnections, ClientsGameData, Cracker, Leaderboard};

use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

pub async fn client_connection_handler(
    ws: WebSocket,
    client_connections: ClientConnections,
    clients_game_data: ClientsGameData,
    cracker: Cracker,
    leaderboard: Leaderboard,
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
        client_id: uuid.clone(),
        friendly_name: "[NO_NAME]".to_string(),
        color: "red".to_string(),
        quack_pitch: 1.0,
        x_pos: PLAYER_X_DEFAULT_START_POSTION,
        y_pos: PLAYER_Y_DEFAULT_START_POSTION,
        direction_facing: DuckDirection::Right,
        radius: PLAYER_RADIUS,
        cracker_count: 0,
        leaderboard_position: 0,
    };

    clients_game_data
        .lock()
        .await
        .insert(uuid.clone(), new_client_game_data);

    // Use a `loop` with an error handling block to ensure cleanup happens
    let disconnect_reason = loop {
        match client_ws_rcv.next().await {
            Some(Ok(msg)) => {
                // Process the message
                let _result = client_msg_handler(
                    &uuid,
                    msg,
                    &client_connections,
                    &clients_game_data,
                    &cracker,
                    &leaderboard,
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

        recalculate_leaderboard_positions(&clients_game_data, &leaderboard).await;

        // Tell other players that user disconnected
        for (_, tx) in client_connections.lock().await.iter() {
            if &tx.client_id != &uuid {
                // send disconnect message
                let user_disconnected_msg = build_user_disconnected_msg(&uuid);

                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(user_disconnected_msg))
                    .unwrap();

                // send leaderboard update message
                let leaderboard_update_msg =
                    build_leaderboard_update_msg(&tx.client_id, &clients_game_data, &leaderboard)
                        .await;

                // Send same leaderboard update message to all players
                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(leaderboard_update_msg))
                    .unwrap();
            }
        }
    }
}
