use warp::filters::ws::Message;

use crate::{
    quackers_game::{
        cracker_creator::generate_random_cracker_data,
        types::{
            defaults::{MAX_X_POS, MAX_Y_POS, MIN_X_POS, MIN_Y_POS},
            game_state::ClientGameData,
            got_crackers_msg::{GotCrackerResponseData, YouGotCrackerMsg},
            msg::OutgoingGameActionType,
            player_move_msg::{MoveResponseData, OtherMovedMsg, YouMovedMsg},
            quack_msg::{QuackResponseData, YouQuackedMsg},
        },
    },
    ClientConnections, ClientsGameData, Cracker,
};

pub async fn handle_move_action(
    sender_client_id: &str,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
    cracker: &Cracker,
) {
    // TODO - unpack request, pull out move direction

    // let did_player_die = check_if_player_died(clients_game_data_arc_mutex);
    let found_cracker =
        check_if_player_touched_crackers(sender_client_id, clients_game_data_arc_mutex, cracker)
            .await;

    // No need to unpack the request data
    let moved_player =
        try_to_move_player(sender_client_id, clients_game_data_arc_mutex, 12, 14).await;

    // Send move message (and found cracker message, if cracker was found) to connected clients
    for (_, tx) in client_connections_arc_mutex.lock().await.iter() {
        if &tx.client_id == sender_client_id {
            // Client that send initial request message

            if let Some(moved_player_data) = &moved_player {
                let you_moved_msg = build_you_moved_msg(moved_player_data).await;

                tx.sender.as_ref().unwrap().send(Ok(you_moved_msg)).unwrap();
            }

            if let Some(cracker) = &found_cracker {
                let you_got_cracker_msg = build_you_got_cracker_msg(&cracker).await;

                tx.sender
                    .as_ref()
                    .unwrap()
                    .send(Ok(you_got_cracker_msg))
                    .unwrap();
            }
        } else {
            // Other Players

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
    }
}

async fn check_if_player_touched_crackers(
    client_id: &str,
    clients_game_data_arc_mutex: &ClientsGameData,
    cracker: &Cracker,
) -> Option<GotCrackerResponseData> {
    let mut cracker_lock = cracker.lock().await;

    let mut client_game_datagaurd = clients_game_data_arc_mutex.lock().await;

    let mut default_game_data = ClientGameData {
        client_id: "error".to_string(),
        x_pos: 0,
        y_pos: 0,
        radius: 0,
        friendly_name: "error".to_string(),
        color: "error".to_string(),
        quack_pitch: 0.,
        cracker_count: 0,
    };

    let client: &mut ClientGameData =
        client_game_datagaurd.get_mut(client_id).unwrap_or_else(|| {
            println!("Couldn't find client with id: {}", client_id);
            &mut default_game_data
        });

    // check if duck is close to crackers
    // good old pythagorean theorem!
    let x_squared: f64 = (&client.x_pos - cracker_lock.x_pos).pow(2) as f64;
    let y_squared: f64 = (client.y_pos - cracker_lock.y_pos).pow(2) as f64;

    let distance: f64 = (x_squared + y_squared).sqrt();

    // got crackers!
    if distance < ((client.radius + cracker_lock.radius) as f64) {
        println!("User {:?} getting crackers!", client.friendly_name);

        let old_cracker_points = cracker_lock.points.clone();
        let old_cracker_points = cracker_lock.points.clone();
        let old_cracker_pos_x = cracker_lock.x_pos.clone();
        let old_cracker_pos_y = cracker_lock.y_pos.clone();

        client.cracker_count += old_cracker_points;

        let cracker_response_data = GotCrackerResponseData {
            player_uuid: client.client_id.clone(),
            player_friendly_name: client.friendly_name.clone(),
            old_cracker_x_position: old_cracker_pos_x,
            old_cracker_y_position: old_cracker_pos_y,
            cracker_point_value: old_cracker_points,
            new_player_score: client.cracker_count,
            new_cracker_x_position: cracker_lock.x_pos,
            new_cracker_y_position: cracker_lock.y_pos,
        };

        // create a new cracker and save it
        *cracker_lock = generate_random_cracker_data();

        return Some(cracker_response_data);
    }

    None
}

async fn try_to_move_player(
    client_id: &str,
    clients_game_data_arc_mutex: &ClientsGameData,
    client_move_intent_x: u64,
    client_move_intent_y: u64,
) -> Option<MoveResponseData> {
    let mut client_game_datagaurd = clients_game_data_arc_mutex.lock().await;

    // let mut default_game_data = ClientGameData {
    //     client_id: "error".to_string(),
    //     x_pos: 0,
    //     y_pos: 0,
    //     radius: 0,
    //     friendly_name: "error".to_string(),
    //     color: "error".to_string(),
    //     quack_pitch: 0.,
    //     cracker_count: 0,
    // };

    // let client: &mut ClientGameData =
    //     client_game_datagaurd.get_mut(&client_id).unwrap_or_else(|| {
    //         println!("Couldn't find client with id: {}", client_id);
    //         &mut default_game_data
    //     });

    // Get a mutable handle to client that moved
    if let Some(client) = client_game_datagaurd.get_mut(client_id) {
        let old_client_x_pos = client.x_pos.clone();
        let old_client_y_pos = client.y_pos.clone();

        // move player
        client.x_pos += client_move_intent_x;
        client.y_pos += client_move_intent_y;

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

        // check if duck is close to crackers
        // good old pythagorean theorem!

        // let mut cracker_lock = cracker.lock().await;

        // let x_squared: f64 =
        //     (client.x_pos - cracker_lock.x_pos).pow(2) as f64;
        // let y_squared: f64 =
        //     (client.y_pos - cracker_lock.y_pos).pow(2) as f64;

        // let distance: f64 = (x_squared + y_squared).sqrt();
    }

    None

    // let you_got_cracker_message_struct = MoveResponseData {
    //     action_type: OutgoingGameActionType::YouMoved,
    //     data: MoveResponseData {
    //         player_uuid: client_id,
    //         player_friendly_name: String,
    //         color: String,
    //         old_x_position: u64,
    //         old_y_position: u64,
    //         new_x_position: u64,
    //         new_y_position: u64,
    //     },
    // };

    // let you_got_cracker_msg_string = serde_json::ser::to_string(&you_got_cracker_message_struct)
    //     .unwrap_or_else(|op| {
    //         println!("Couldn't convert YouGotCracker struct to string");
    //         "".to_string()
    //     });

    // Message::text(you_got_cracker_msg_string)
}

async fn build_you_moved_msg(you_moved_response_data: &MoveResponseData) -> Message {
    let you_moved_message_struct = YouMovedMsg {
        action_type: OutgoingGameActionType::YouMoved,
        data: you_moved_response_data.clone(),
    };

    let you_moved_msg_string = serde_json::ser::to_string(&you_moved_message_struct)
        .unwrap_or_else(|op| {
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
        .unwrap_or_else(|op| {
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
        serde_json::ser::to_string(&other_player_moved_message_struct).unwrap_or_else(|op| {
            println!("Couldn't convert OtherPlayerMoved struct to string");
            "".to_string()
        });

    Message::text(other_player_moved_msg_string)
}

async fn build_other_player_got_cracker_msg(
    got_cracker_response_data: &GotCrackerResponseData,
) -> Message {
    let other_player_got_cracker_message_struct = YouGotCrackerMsg {
        action_type: OutgoingGameActionType::OtherPlayerGotCrackers,
        data: got_cracker_response_data.clone(),
    };

    let other_player_got_cracker_msg_string =
        serde_json::ser::to_string(&other_player_got_cracker_message_struct).unwrap_or_else(|op| {
            println!("Couldn't convert OtherPlayerGotCracker struct to string");
            "".to_string()
        });

    Message::text(other_player_got_cracker_msg_string)
}

async fn build_other_player_quacked_msg(
    quacker_client_id: &str,
    quacker_clients_game_data: &ClientsGameData,
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
            println!("Couldn't convert Other Player Quacked struct to string");
            "".to_string()
        });

    Message::text(quack_message_string)
}
