use warp::filters::ws::Message;

use crate::{
    quackers_game::{
        game::{
            cracker_creator::generate_random_cracker_data,
            game_constants::{MAX_X_POS, MAX_Y_POS, MIN_X_POS, MIN_Y_POS},
            game_state::{ClientGameData, DuckDirection},
        },
        messages::{
            join::receive_submit_name_request::{
                build_leaderboard_update_msg, recalculate_leaderboard_positions,
            }, msg_types::{GenericIncomingRequest, OutgoingGameActionType}, player_move::{
                getting_crackers::getting_crackers_msg::build_other_player_got_cracker_msg,
                player_move_types::MoveRequestData,
            }
        }
    },
    ClientConnections, ClientsGameData, Cracker, Leaderboard,
};

use super::{
    getting_crackers::getting_crackers_types::{GotCrackerResponseData, YouGotCrackerMsg},
    player_move_types::{MoveResponseData, OtherMovedMsg, YouMovedMsg},
};

pub async fn handle_move_action(
    sender_client_id: &str,
    json_message: GenericIncomingRequest,
    client_connections_mutex: &ClientConnections,
    clients_game_data_mutex: &ClientsGameData,
    cracker_mutex: &Cracker,
    leaderboard_mutex: &Leaderboard,
) {
    println!("Move request received: {:?}", &json_message.data);

    // Unpack the move request
    let move_request_data: MoveRequestData = serde_json::from_value(json_message.data)
        .unwrap_or_else(|err| {
            println!("Couldn't convert data to MoveRequestData struct: {:?}", err);
            MoveRequestData {
                x_direction: 0.,
                y_direction: 0.,
            }
        });

    // TODO - Add dangerous objects so players can die?

    let found_cracker =
        check_if_player_touched_crackers(sender_client_id, clients_game_data_mutex, cracker_mutex)
            .await;

    let moved_player = try_to_move_player(
        sender_client_id,
        clients_game_data_mutex,
        &move_request_data,
    )
    .await;

    // Send move message (and found cracker message, if cracker was found) to connected clients
    for (_, tx) in client_connections_mutex.lock().await.iter() {
        if &tx.client_id == sender_client_id {
            // Send move msg back to client that send initial request message

            if let Some(moved_player_data) = &moved_player {
                let you_moved_msg = build_you_moved_msg(moved_player_data).await;
                tx.sender.as_ref().unwrap().send(Ok(you_moved_msg)).unwrap();
            }

            if let Some(cracker) = &found_cracker {
                println!("Building you got crackers message");
                let you_got_cracker_msg = build_you_got_cracker_msg(&cracker).await;

                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(you_got_cracker_msg))
                    .unwrap();
            }
        } else {
            // Send move message to other players

            if let Some(moved_player_data) = &moved_player {
                let other_player_moved_msg = build_other_player_moved_msg(moved_player_data).await;
                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(other_player_moved_msg))
                    .unwrap();
            }

            if let Some(cracker) = &found_cracker {
                let other_player_got_cracker_msg =
                    build_other_player_got_cracker_msg(&cracker).await;

                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(other_player_got_cracker_msg))
                    .unwrap();
            }
        }

        // If moved into a cracker, recalc leaderboard and send update message to everyone
        if let Some(_cracker) = &found_cracker {
            println!("Found a cracker, recalculating leaderboard...");

            // TODO - Don't recalc on every iteration of loop?
            recalculate_leaderboard_positions(&clients_game_data_mutex, &leaderboard_mutex).await;

            let leaderboard_update_msg = build_leaderboard_update_msg(
                &tx.client_id,
                &clients_game_data_mutex,
                &leaderboard_mutex,
            )
            .await;

            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(leaderboard_update_msg))
                .unwrap();
        }
    }
}

// TODO -test this
async fn check_if_player_touched_crackers(
    client_id: &str,
    clients_game_data_arc_mutex: &ClientsGameData,
    cracker: &Cracker,
) -> Option<GotCrackerResponseData> {
    let mut cracker_lock = cracker.lock().await;

    let mut client_game_datagaurd = clients_game_data_arc_mutex.lock().await;

    let mut default_game_data = ClientGameData {
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

    let client: &mut ClientGameData =
        client_game_datagaurd.get_mut(client_id).unwrap_or_else(|| {
            println!("Couldn't find client with id: {}", client_id);
            &mut default_game_data
        });

    // check if duck is close to crackers
    // good old pythagorean theorem!
    let x_squared: f32 = (&client.x_pos - cracker_lock.x_pos).powf(2.) as f32;
    let y_squared: f32 = (client.y_pos - cracker_lock.y_pos).powf(2.) as f32;

    let distance: f32 = (x_squared + y_squared).sqrt();

    let combined_radii = client.radius + cracker_lock.radius;

    println!("Client x: {} y: {}", &client.x_pos, &client.y_pos);

    println!(
        "Crackers x: {} y: {}",
        &cracker_lock.x_pos, &cracker_lock.y_pos
    );

    println!(
        "Comparing distance from cracker: {} to combined radii: {}",
        distance, combined_radii
    );

    // got crackers!
    if distance < (combined_radii as f32) {
        let old_cracker_points = cracker_lock.points.clone();
        let old_cracker_pos_x = cracker_lock.x_pos.clone();
        let old_cracker_pos_y = cracker_lock.y_pos.clone();

        client.cracker_count += old_cracker_points;

        println!(
            "User {:?} getting crackers! new score: {}",
            client.friendly_name, client.cracker_count
        );

        // create a new cracker and save it
        *cracker_lock = generate_random_cracker_data();

        let cracker_response_data = GotCrackerResponseData {
            player_uuid: client.client_id.clone(),
            player_friendly_name: client.friendly_name.clone(),
            old_cracker_x_position: old_cracker_pos_x,
            old_cracker_y_position: old_cracker_pos_y,
            old_cracker_point_value: old_cracker_points,
            new_cracker_point_value: cracker_lock.points,
            new_player_score: client.cracker_count,
            new_cracker_x_position: cracker_lock.x_pos,
            new_cracker_y_position: cracker_lock.y_pos,
        };

        return Some(cracker_response_data);
    }

    None
}

// TODO - rename this better
async fn try_to_move_player(
    client_id: &str,
    clients_game_data_arc_mutex: &ClientsGameData,
    move_request_data: &MoveRequestData,
) -> Option<MoveResponseData> {
    let mut client_game_datagaurd = clients_game_data_arc_mutex.lock().await;

    // Get a mutable handle to client that moved
    if let Some(client) = client_game_datagaurd.get_mut(client_id) {
        let old_client_x_pos = client.x_pos.clone();
        let old_client_y_pos = client.y_pos.clone();

        // normalize to 10
        let mut normalized_x = move_request_data.x_direction;
        if normalized_x > 10. {
            normalized_x = 10.
        };
        if normalized_x < -10. {
            normalized_x = -10.
        };
        let mut normalized_y = move_request_data.y_direction;
        if normalized_y > 10. {
            normalized_y = 10.
        };
        if normalized_y < -10. {
            normalized_y = -10.
        };

        client.x_pos += normalized_x;
        client.y_pos += normalized_y;

        // keep within bounds, though
        if client.x_pos > MAX_X_POS {
            client.x_pos = MAX_X_POS;
        }
        if client.x_pos < MIN_X_POS {
            client.x_pos = MIN_X_POS;
        }
        if client.y_pos > MAX_Y_POS {
            client.y_pos = MAX_Y_POS;
        }
        if client.y_pos < MIN_Y_POS {
            client.y_pos = MIN_Y_POS;
        }

        // set duck direction
        if move_request_data.x_direction > 0. {
            client.direction_facing = DuckDirection::Right
        } else if move_request_data.x_direction < 0. {
            client.direction_facing = DuckDirection::Left
        }

        let you_moved_msg_data = MoveResponseData {
            player_uuid: client_id.to_string(),
            player_friendly_name: client.friendly_name.to_string(),
            color: client.color.to_string(),
            old_x_position: old_client_x_pos,
            old_y_position: old_client_y_pos,
            new_x_position: client.x_pos,
            new_y_position: client.y_pos,
        };

        return Some(you_moved_msg_data);
    }

    None
}

// TODO -break out these functions to their own files

async fn build_you_moved_msg(you_moved_response_data: &MoveResponseData) -> Message {
    let you_moved_message_struct = YouMovedMsg {
        action_type: OutgoingGameActionType::YouMoved,
        data: you_moved_response_data.clone(),
    };

    let you_moved_msg_string = serde_json::ser::to_string(&you_moved_message_struct)
        .unwrap_or_else(|_op| {
            println!("Couldn't convert YouMoved struct to string");
            "".to_string()
        });

    Message::text(you_moved_msg_string)
}

async fn build_you_got_cracker_msg(got_cracker_response_data: &GotCrackerResponseData) -> Message {
    let you_got_cracker_message_struct = YouGotCrackerMsg {
        action_type: OutgoingGameActionType::YouGotCrackers,
        data: got_cracker_response_data.clone(),
    };

    let you_got_cracker_msg_string = serde_json::ser::to_string(&you_got_cracker_message_struct)
        .unwrap_or_else(|_op| {
            println!("Couldn't convert YouGotCracker struct to string");
            "".to_string()
        });

    Message::text(you_got_cracker_msg_string)
}

async fn build_other_player_moved_msg(move_response_data: &MoveResponseData) -> Message {
    let other_player_moved_message_struct = OtherMovedMsg {
        action_type: OutgoingGameActionType::OtherPlayerMoved,
        data: move_response_data.clone(),
    };

    let other_player_moved_msg_string =
        serde_json::ser::to_string(&other_player_moved_message_struct).unwrap_or_else(|_op| {
            println!("Couldn't convert OtherPlayerMoved struct to string");
            "".to_string()
        });

    Message::text(other_player_moved_msg_string)
}
