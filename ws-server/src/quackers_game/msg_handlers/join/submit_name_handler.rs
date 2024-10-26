use std::cmp::Ordering;
use std::convert::TryFrom;

use rand::thread_rng;
use rand::Rng;
use warp::filters::ws::Message;

use crate::quackers_game::game::game_state::ClientGameData;
use crate::quackers_game::game::game_state::LeaderboardData;
use crate::quackers_game::types::defaults::AVAILABLE_NAMES;
use crate::quackers_game::types::leaderboard_update_msg::{
    LeaderboardUpdateData, LeaderboardUpdateMsg,
};
use crate::quackers_game::types::player_join_msg::{
    DuckDirection, NewJoinerDataWithAllPlayers, OtherPlayerData,
};
use crate::{
    quackers_game::types::{
        defaults::AVAILABLE_DUCK_COLORS_WEIGHTED,
        msg::{GenericIncomingRequest, OutgoingGameActionType},
        player_join_msg::{JoinRequestData, OtherPlayerJoinedMsg, YouJoinedMsg},
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

        let mut rng = thread_rng();

        let randomly_chosen_name = match AVAILABLE_NAMES.choose(&mut rng) {
            Some(random_element) => random_element,
            _ => "Guest",
        };

        let randomly_chosen_color = weighted_choose(&AVAILABLE_DUCK_COLORS_WEIGHTED);
        println!("Randomly chosen color: {}", randomly_chosen_color);

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

pub fn weighted_choose<T: Copy>(options: &[(T, u32)]) -> T {
    let mut rng = thread_rng();
    let total_weight: u32 = options.iter().map(|&(_, weight)| weight).sum();
    let mut random_value = rng.gen_range(0..total_weight);

    for &(item, weight) in options {
        if random_value < weight {
            return item; // Return the selected item
        }
        random_value -= weight; // Reduce the random_value by the current weight
    }

    // Fallback case, should never hit here if weights are set correctly
    options[0].0 // Return first option if all else fails
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
    // clients_game_data_vec.sort_by(|a, b| b.1.cracker_count.cmp(&a.1.cracker_count));

    clients_game_data_vec.sort_by(|a, b| {
        let count_cmp = b.1.cracker_count.cmp(&a.1.cracker_count); // Compare cracker_count
        if count_cmp == Ordering::Equal {
            // If counts are equal, reverse the original order
            a.0.cmp(b.0) // Change to a.0.cmp(b.0) if you want ascending order for names
        } else {
            count_cmp // Otherwise, return the comparison result of cracker_count
        }
    });

    // clients_game_data_vec
    //     .sort_by(|a: &(&String, &ClientGameData), b: &(&String, &ClientGameData)| b.1.cracker_count.cmp(&a.1.cracker_count));

    // Print the sorted results
    // for (client_name, game_data) in clients_game_data_vec {
    //     println!("New sorting: {}: {}", client_name, game_data.cracker_count);
    // }

    let mut leaderboard_gaurd = leaderboard.lock().await;

    // clear out all leaderboard slots fi we need to
    let clients_connected_length = clients_game_data_vec.len();

    if clients_connected_length == 0 {
        leaderboard_gaurd.leaderboard_name_1st_place = "--".to_string();
        leaderboard_gaurd.leaderboard_score_1st_place = 0;
    }

    if clients_connected_length <= 1 {
        leaderboard_gaurd.leaderboard_name_2nd_place = "--".to_string();
        leaderboard_gaurd.leaderboard_score_2nd_place = 0;
    }

    if clients_connected_length <= 2 {
        leaderboard_gaurd.leaderboard_name_3rd_place = "--".to_string();
        leaderboard_gaurd.leaderboard_score_3rd_place = 0;
    }

    if clients_connected_length <= 3 {
        leaderboard_gaurd.leaderboard_name_4th_place = "--".to_string();
        leaderboard_gaurd.leaderboard_score_4th_place = 0;
    }

    if clients_connected_length <= 4 {
        leaderboard_gaurd.leaderboard_name_5th_place = "--".to_string();
        leaderboard_gaurd.leaderboard_score_5th_place = 0;
    }

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

pub async fn build_you_joined_msg(
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
        direction_facing: DuckDirection::Right,
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

    // let all_other_players = [];

    // let mut clients_game_data_gaurd = clients_game_data_arc_mutex.lock().await;
    // let mut clients_game_data_vec: Vec<&mut ClientGameData> =
    // gaurd.iter_mut().map(|(_str, &mut game_data)| {
    //     game_data
    // }).collect();

    let clients_game_data_vec: Vec<OtherPlayerData> = gaurd
        .iter()
        .map(|(_str, game_data)| OtherPlayerData {
            player_uuid: game_data.client_id.clone(),
            player_friendly_name: game_data.friendly_name.clone(),
            color: game_data.color.clone(),
            x_position: game_data.x_pos,
            y_position: game_data.y_pos,
            direction_facing: game_data.direction_facing.clone(),
        }) // game_data is already &mut ClientGameData
        .filter(|other_player_data| other_player_data.player_uuid != sender_game_data.client_id)
        .collect();

    let message_struct = YouJoinedMsg {
        action_type: OutgoingGameActionType::YouJoined,
        data: NewJoinerDataWithAllPlayers {
            player_uuid: sender_game_data.client_id.clone(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            color: sender_game_data.color.clone(),
            x_position: sender_game_data.x_pos,
            y_position: sender_game_data.x_pos,
            cracker_x: cracker_gaurd.x_pos,
            cracker_y: cracker_gaurd.y_pos,
            cracker_points: cracker_gaurd.points,
            player_points: sender_game_data.cracker_count,
            all_other_players: clients_game_data_vec,
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
        direction_facing: DuckDirection::Right,
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
) -> Message {
    let default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0.,
        y_pos: 0.,
        direction_facing: DuckDirection::Right,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
        leaderboard_position: 0,
    };

    let gaurd = joiner_clients_game_data.lock().await;

    let sender_game_data = gaurd.get(joiner_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", joiner_client_id);
        &default_game_data
    });

    let message_struct = OtherPlayerJoinedMsg {
        action_type: OutgoingGameActionType::OtherPlayerJoined,
        data: OtherPlayerData {
            player_uuid: sender_game_data.client_id.to_string(),
            player_friendly_name: sender_game_data.friendly_name.clone(),
            color: sender_game_data.color.clone(),
            x_position: sender_game_data.x_pos,
            y_position: sender_game_data.y_pos,
            direction_facing: sender_game_data.direction_facing.clone(),
            // cracker_x: cracker_gaurd.x_pos,
            // cracker_y: cracker_gaurd.y_pos,
            // cracker_points: cracker_gaurd.points,
            // player_points: sender_game_data.cracker_count,
        },
    };

    let message_string = serde_json::ser::to_string(&message_struct).unwrap_or_else(|_op| {
        println!("Couldn't convert Other Player Joined struct to string");
        "".to_string()
    });

    Message::text(message_string)
}
