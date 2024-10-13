use std::convert::TryFrom;
use std::{collections::HashMap, hash::RandomState, sync::Mutex};

use rand::thread_rng;
use warp::filters::ws::Message;

use crate::quackers_game::types::game_state::LeaderboardData;
use crate::quackers_game::types::leaderboard_update_msg::{
    self, LeaderboardUpdateData, LeaderboardUpdateMsg,
};
use crate::{
    quackers_game::types::{
        defaults::AVAILABLE_DUCK_COLORS,
        game_state::{ClientConnection, ClientGameData},
        msg::{GenericIncomingRequest, OutgoingGameActionType},
        player_join_msg::{JoinRequestData, NewJoinerData, OtherPlayerJoinedMsg, YouJoinedMsg},
    },
    ClientConnections, ClientsGameData, Cracker, Leaderboard,
};

use rand::prelude::SliceRandom;

pub async fn handle_submit_name_action(
    sender_client_id: &str,
    json_message: GenericIncomingRequest,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
    cracker_mutex: &Cracker,
    leaderboard_mutex: &Leaderboard,
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

    // Mutates BOTH clients_game_data_arc_mutex (position) And leaderboard_mutex!!
    recalculate_leaderboard_positions(clients_game_data_arc_mutex, leaderboard_mutex).await;

    // Tell everyone about new user join and leaderboard update
    for (_, tx) in client_connections_arc_mutex.lock().await.iter() {
        // YOU joined
        if &tx.client_id == sender_client_id {
            let you_joined_msg = build_you_joined_msg(
                sender_client_id,
                &clients_game_data_arc_mutex,
                &cracker_mutex,
            )
            .await;

            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(you_joined_msg))
                .unwrap();
        } else {
            // OTHER player joiend
            let other_player_joined_msg = build_other_player_joined_msg(
                sender_client_id,
                &clients_game_data_arc_mutex,
                &cracker_mutex,
            )
            .await;
            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(other_player_joined_msg))
                .unwrap();
        }

        let leaderboard_update_msg = build_leaderboard_update_msg(
            &tx.client_id,
            clients_game_data_arc_mutex,
            leaderboard_mutex,
        )
        .await;

        // Send same leaderboard update message to all players
        tx.sender
            .as_ref()
            .unwrap()
            .send(Ok(leaderboard_update_msg))
            .unwrap();
    }
}

pub async fn recalculate_leaderboard_positions(
    clients_game_data_arc_mutex: &ClientsGameData,
    leaderboard: &Leaderboard,
) -> LeaderboardData {
    // for (_, client_game_data) in
    let mut clients_game_data_gaurd = clients_game_data_arc_mutex.lock().await;
    let mut clients_game_data_vec: Vec<(&String, &mut ClientGameData)> =
        clients_game_data_gaurd.iter_mut().collect();

    // Sorts clients, mutating vetor in place
    // Sort the clients in descending order based on cracker_count
    clients_game_data_vec.sort_by(|a, b| b.1.cracker_count.cmp(&a.1.cracker_count));

    // clients_game_data_vec
    //     .sort_by(|a: &(&String, &ClientGameData), b: &(&String, &ClientGameData)| b.1.cracker_count.cmp(&a.1.cracker_count));

    // Print the sorted results
    // for (client_name, game_data) in clients_game_data_vec {
    //     println!("New sorting: {}: {}", client_name, game_data.cracker_count);
    // }

    let mut leaderboard_gaurd = leaderboard.lock().await;

    for (index, (_uuid, client_game_data)) in clients_game_data_vec.iter_mut().enumerate() {
        if let Ok(index_u64) = u64::try_from(index) {
            // if let Some(client) = clients_game_data_gaurd.get_mut(_uuid.as_str()) {
            //     client.leaderboard_position = index_u64 + 1;
            // }
            client_game_data.leaderboard_position = index_u64 + 1;
            println!(
                "Updating position for {}! value: {}",
                client_game_data.friendly_name, client_game_data.leaderboard_position
            );
        } else {
            println!("Failed to convert index to u64");
        }

        if index == 0 {
            leaderboard_gaurd.leaderboard_name_1st_place = client_game_data.friendly_name.clone();
            leaderboard_gaurd.leaderboard_score_1st_place = client_game_data.cracker_count;
        }

        if index == 1 {
            leaderboard_gaurd.leaderboard_name_2nd_place = client_game_data.friendly_name.clone();
            leaderboard_gaurd.leaderboard_score_2nd_place = client_game_data.cracker_count;
        }

        if index == 2 {
            leaderboard_gaurd.leaderboard_name_3rd_place = client_game_data.friendly_name.clone();
            leaderboard_gaurd.leaderboard_score_3rd_place = client_game_data.cracker_count;
        }

        if index == 3 {
            leaderboard_gaurd.leaderboard_name_4th_place = client_game_data.friendly_name.clone();
            leaderboard_gaurd.leaderboard_score_4th_place = client_game_data.cracker_count;
        }

        if index == 4 {
            leaderboard_gaurd.leaderboard_name_5th_place = client_game_data.friendly_name.clone();
            leaderboard_gaurd.leaderboard_score_5th_place = client_game_data.cracker_count;
        }
    }

    return LeaderboardData {
        leaderboard_name_1st_place: leaderboard_gaurd.leaderboard_name_1st_place.clone(),
        leaderboard_name_2nd_place: leaderboard_gaurd.leaderboard_name_2nd_place.clone(),
        leaderboard_name_3rd_place: leaderboard_gaurd.leaderboard_name_3rd_place.clone(),
        leaderboard_name_4th_place: leaderboard_gaurd.leaderboard_name_4th_place.clone(),
        leaderboard_name_5th_place: leaderboard_gaurd.leaderboard_name_5th_place.clone(),
        leaderboard_score_1st_place: leaderboard_gaurd.leaderboard_score_1st_place,
        leaderboard_score_2nd_place: leaderboard_gaurd.leaderboard_score_2nd_place,
        leaderboard_score_3rd_place: leaderboard_gaurd.leaderboard_score_3rd_place,
        leaderboard_score_4th_place: leaderboard_gaurd.leaderboard_score_4th_place,
        leaderboard_score_5th_place: leaderboard_gaurd.leaderboard_score_5th_place,
    };
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
        leaderboard_position: 0,
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
            player_points: sender_game_data.cracker_count,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert You Joined struct to string");
        "".to_string()
    });

    Message::text(message_string)
}

pub async fn build_leaderboard_update_msg(
    client_id: &str,
    clients_game_data_arc_mutex: &ClientsGameData,
    leaderboard_mutex: &Leaderboard,
) -> Message {
    let clients_game_data_gaurd = clients_game_data_arc_mutex.lock().await;
    let leaderboard_gaurd = leaderboard_mutex.lock().await;

    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0.,
        y_pos: 0.,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
        leaderboard_position: 0,
    };

    let client_data = clients_game_data_gaurd.get(client_id).unwrap_or_else(|| {
        println!("Couldn't find data for client!");
        &default_game_data
    });

    let message_struct = LeaderboardUpdateMsg {
        action_type: OutgoingGameActionType::LeaderboardUpdate,
        data: LeaderboardUpdateData {
            your_points: client_data.cracker_count,
            your_leaderboard_place: client_data.leaderboard_position,
            leaderboard_name_1st_place: leaderboard_gaurd.leaderboard_name_1st_place.clone(),
            leaderboard_name_2nd_place: leaderboard_gaurd.leaderboard_name_2nd_place.clone(),
            leaderboard_name_3rd_place: leaderboard_gaurd.leaderboard_name_3rd_place.clone(),
            leaderboard_name_4th_place: leaderboard_gaurd.leaderboard_name_4th_place.clone(),
            leaderboard_name_5th_place: leaderboard_gaurd.leaderboard_name_5th_place.clone(),
            leaderboard_score_1st_place: leaderboard_gaurd.leaderboard_score_1st_place,
            leaderboard_score_2nd_place: leaderboard_gaurd.leaderboard_score_2nd_place,
            leaderboard_score_3rd_place: leaderboard_gaurd.leaderboard_score_3rd_place,
            leaderboard_score_4th_place: leaderboard_gaurd.leaderboard_score_4th_place,
            leaderboard_score_5th_place: leaderboard_gaurd.leaderboard_score_5th_place,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert LeaderboardUpdateDataMsg struct to string");
        "".to_string()
    });

    println!("Leaderboard message going out: {}", message_string);

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
        leaderboard_position: 0,
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
            player_points: sender_game_data.cracker_count,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert Other Player Joined struct to string");
        "".to_string()
    });

    Message::text(message_string)
}
