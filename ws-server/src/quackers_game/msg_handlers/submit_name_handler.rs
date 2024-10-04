use std::convert::TryInto;

use warp::filters::ws::Message;

use crate::{
    quackers_game::types::{
        game_state::ClientGameData,
        msg::{GenericIncomingRequest, OutgoingGameActionType},
        player_join_msg::JoinRequestData,
        quack_msg::{QuackResponseData, YouQuackedMsg},
    },
    ClientConnections, ClientsGameData,
};

pub async fn handle_submit_name_action(
    sender_client_id: &str,
    json_message: GenericIncomingRequest,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
) {
    println!("converting json_message.data {}", json_message.data);
    let submit_action_request_data: JoinRequestData = serde_json::from_value(json_message.data)
        .unwrap_or_else(|err| {
            println!("Couldn't convert data to JoinRequestData struct");
            JoinRequestData {
                friendly_name: "".to_string(),
            }
        });

    println!("Finished converting");

    if let Some(mutable_game_data_gaurd) = clients_game_data_arc_mutex
        .lock()
        .await
        .get_mut(sender_client_id)
    {
        println!("mutatng..");
        mutable_game_data_gaurd.friendly_name = submit_action_request_data.friendly_name;
    };

    println!("k1");

    println!("k2");
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

    println!("k3");
}

async fn build_you_quacked_msg(
    quacker_client_id: &str,
    // quackerClient: &ClientConnection,
    quackerClientsGameData: &ClientsGameData,
) -> Message {
    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0,
        y_pos: 0,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let gaurd = quackerClientsGameData.lock().await;

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
    // quackerClient: &ClientConnection,
    quackerClientsGameData: &ClientsGameData,
) -> Message {
    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0,
        y_pos: 0,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let gaurd = quackerClientsGameData.lock().await;

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
