use warp::filters::ws::Message;

use crate::{
    quackers_game::types::{
        game_state::ClientGameData,
        msg::OutgoingGameActionType,
        quack_msg::{QuackResponseData, YouQuackedMsg},
    },
    ClientConnections, ClientsGameData,
};

pub async fn handle_quack_action(
    sender_client_id: &str,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
) {
    // No need to unpack the request data

    // Send quack message to connected clients
    for (_, tx) in client_connections_arc_mutex.lock().await.iter() {
        if &tx.client_id == sender_client_id {
            let you_quacked_msg =
                build_you_quacked_msg(sender_client_id, &clients_game_data_arc_mutex).await;

            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(you_quacked_msg))
                .unwrap();
        } else {
            let other_player_quacked_msg =
                build_other_player_quacked_msg(sender_client_id, &clients_game_data_arc_mutex)
                    .await;
            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(other_player_quacked_msg))
                .unwrap();
        }
    }
}

async fn build_you_quacked_msg(
    quacker_client_id: &str,
    quacker_clients_game_data: &ClientsGameData,
) -> Message {
    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0.,
        y_pos: 0.,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let gaurd = quacker_clients_game_data.lock().await;

    let sender_game_data = gaurd.get(quacker_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", quacker_client_id);
        &default_game_data
    });

    let quack_message_struct = YouQuackedMsg {
        action_type: OutgoingGameActionType::YouQuacked,
        data: QuackResponseData {
            player_uuid: sender_game_data.client_id.to_string(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            player_x_position: sender_game_data.x_pos,
            player_y_position: sender_game_data.y_pos,
            quack_pitch: sender_game_data.quack_pitch,
        },
    };

    let quack_message_string =
        serde_json::ser::to_string(&quack_message_struct).unwrap_or_else(|op| {
            println!("Couldn't convert You Quacked struct to string");
            "".to_string()
        });

    Message::text(quack_message_string)
}

async fn build_other_player_quacked_msg(
    quacker_client_id: &str,
    quacker_clients_game_data: &ClientsGameData,
) -> Message {
    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0.,
        y_pos: 0.,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let gaurd = quacker_clients_game_data.lock().await;

    let sender_game_data = gaurd.get(quacker_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", quacker_client_id);
        &default_game_data
    });

    let quack_message_struct = YouQuackedMsg {
        action_type: OutgoingGameActionType::OtherPlayerQuacked,
        data: QuackResponseData {
            player_uuid: sender_game_data.client_id.to_string(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            player_x_position: sender_game_data.x_pos,
            player_y_position: sender_game_data.y_pos,
            quack_pitch: sender_game_data.quack_pitch,
        },
    };

    let quack_message_string =
        serde_json::ser::to_string(&quack_message_struct).unwrap_or_else(|op| {
            println!("Couldn't convert You Quacked struct to string");
            "".to_string()
        });

    Message::text(quack_message_string)
}
