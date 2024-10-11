use rand::thread_rng;
use warp::filters::ws::Message;

use crate::{
    quackers_game::types::{
        defaults::AVAILABLE_DUCK_COLORS,
        game_state::ClientGameData,
        msg::{GenericIncomingRequest, OutgoingGameActionType},
        player_join_msg::{JoinRequestData, NewJoinerData, OtherPlayerJoinedMsg, YouJoinedMsg},
    },
    ClientConnections, ClientsGameData, Cracker,
};

use rand::prelude::SliceRandom;

pub async fn handle_submit_name_action(
    sender_client_id: &str,
    json_message: GenericIncomingRequest,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
    cracker_mutex: &Cracker,
) {
    // Unpack the request
    let submit_action_request_data: JoinRequestData = serde_json::from_value(json_message.data)
        .unwrap_or_else(|_err| {
            println!("Couldn't convert data to JoinRequestData struct");
            JoinRequestData {
                friendly_name: "".to_string(),
            }
        });

    // Update friendly name for the correct player
    if let Some(mutable_game_data_gaurd) = clients_game_data_arc_mutex
        .lock()
        .await
        .get_mut(sender_client_id)
    {
        println!(
            "mutating user with id: {}, new friendly_name: {}..",
            sender_client_id, submit_action_request_data.friendly_name
        );

        let available_names = vec![
            "Jimbo".to_string(),
            "Chip".to_string(),
            "Francesca".to_string(),
            "Lucy".to_string(),
            "Jerome".to_string(),
            "Phillonius".to_string(),
            "Faran".to_string(),
            "Cory".to_string(),
        ];

        let mut rng = thread_rng();

        let randomly_chosen_name = match available_names.choose(&mut rng) {
            Some(random_element) => random_element,
            _ => "Guest",
        };

        let randomly_chosen_color = match AVAILABLE_DUCK_COLORS.choose(&mut rng) {
            Some(random_element) => random_element,
            _ => "white",
        };

        mutable_game_data_gaurd.friendly_name = randomly_chosen_name.to_string();
        mutable_game_data_gaurd.color = randomly_chosen_color.to_string();
    }

    // Tell everyone about new user join
    for (_, tx) in client_connections_arc_mutex.lock().await.iter() {
        if &tx.client_id == sender_client_id {
            let you_joined_msg =
                build_you_joined_msg(sender_client_id, &clients_game_data_arc_mutex, &cracker_mutex).await;

            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(you_joined_msg))
                .unwrap();
        } else {
            let other_player_joined_msg =
                build_other_player_joined_msg(sender_client_id, &clients_game_data_arc_mutex, &cracker_mutex).await;
            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(other_player_joined_msg))
                .unwrap();
        }
    }
}

async fn build_you_joined_msg(
    joiner_client_id: &str,
    clients_game_data: &ClientsGameData,
    cracker: &Cracker,
) -> Message {

    let gaurd = clients_game_data.lock().await;
    let cracker_gaurd = cracker.lock().await;

    let default = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0.,
        y_pos: 0.,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let sender_game_data = gaurd.get(joiner_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", joiner_client_id);
        &default
    });

    let message_struct = YouJoinedMsg {
        action_type: OutgoingGameActionType::YouJoined,
        data: NewJoinerData {
            player_uuid: sender_game_data.client_id.clone(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            color: sender_game_data.color.clone(),
            x_position: sender_game_data.x_pos,
            y_position: sender_game_data.x_pos,
            cracker_x: cracker_gaurd.x_pos,
            cracker_y: cracker_gaurd.y_pos,
            cracker_points: cracker_gaurd.points,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert You Joined struct to string");
        "".to_string()
    });

    Message::text(message_string)
}

async fn build_other_player_joined_msg(
    joiner_client_id: &str,
    joiner_clients_game_data: &ClientsGameData,
    cracker: &Cracker,
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

    let gaurd = joiner_clients_game_data.lock().await;
    let cracker_gaurd = cracker.lock().await;

    let sender_game_data = gaurd.get(joiner_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", joiner_client_id);
        &default_game_data
    });

    let message_struct = OtherPlayerJoinedMsg {
        action_type: OutgoingGameActionType::OtherPlayerJoined,
        data: NewJoinerData {
            player_uuid: sender_game_data.client_id.to_string(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            color: sender_game_data.color.clone(),
            x_position: sender_game_data.x_pos,
            y_position: sender_game_data.y_pos,
            cracker_x: cracker_gaurd.x_pos,
            cracker_y: cracker_gaurd.y_pos,
            cracker_points: cracker_gaurd.points,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert Other Player Joined struct to string");
        "".to_string()
    });

    Message::text(message_string)
}
