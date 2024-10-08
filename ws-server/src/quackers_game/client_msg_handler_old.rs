use crate::quackers_game_logic::cracker_creator::generate_random_cracker_data;
use crate::quackers_game_logic::types::defaults::{MAX_X_POS, MAX_Y_POS, MIN_X_POS, MIN_Y_POS};
use crate::quackers_game_logic::types::got_crackers_msg::{GotCrackers, GotCrackersData};
use crate::quackers_game_logic::types::msg::{
    GenericIncomingRequest, IncomingGameActionType, OutgoingGameActionType,
};
use crate::quackers_game_logic::types::player_move_msg::{
    MoveRequestData, SomeoneMovedData, SomeoneMovedMsg,
};
use crate::quackers_game_logic::types::quack_msg::{QuackData, SomeoneQuacked};
use crate::{Clients, Cracker};

use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use std::borrow::BorrowMut;
use std::convert::TryInto;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;
// use super::types::*;

pub async fn client_msg(client_id: &str, msg: Message, clients: &Clients, cracker: &Cracker) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let clients_lock = clients.lock().await;

    match clients_lock.get(client_id) {
        Some(request_originator_client) => {
            if let Some(sender) = &request_originator_client.sender {
                println!("Got message from a sender! {:?}", message);

                let json_result: Result<GenericIncomingRequest, _> = serde_json::from_str(message);

                match json_result {
                    Ok(incoming_request) => {
                        println!("got action of type: {}", incoming_request.action_type);

                        match IncomingGameActionType::from_str(&incoming_request.action_type) {
                            Ok(action_type) => {
                                match action_type {
                                    IncomingGameActionType::PlayerMove => {
                                        let incoming_data_json: Result<MoveRequestData, _> =
                                            serde_json::from_value(incoming_request.data);
                                        let incoming_data = incoming_data_json.expect("Couldnt");

                                        let mut clients_lock = clients.lock().await;

                                        // let interable_clients = clients_lock.iter();
                                        if let Some(client) = clients_lock.get_mut(client_id) {
                                            let old_client_x_pos = client.x_pos.clone();
                                            let old_client_y_pos = client.y_pos.clone();

                                            // move player
                                            client.x_pos += incoming_data.x_direction;
                                            client.y_pos += incoming_data.y_direction;

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

                                            // check if duck is close to crackers
                                            // good old pythagorean theorem!

                                            let mut cracker_lock = cracker.lock().await;

                                            let x_squared: f32 =
                                                (client.x_pos - cracker_lock.x_pos).pow(2) as f32;
                                            let y_squared: f32 =
                                                (client.y_pos - cracker_lock.y_pos).pow(2) as f32;

                                            let distance: f32 = (x_squared + y_squared).sqrt();

                                            // got crackers!
                                            if distance
                                                < ((client.radius + cracker_lock.radius) as f32)
                                            {
                                                println!(
                                                    "User {:?} getting crackers!",
                                                    client.friendly_name
                                                );

                                                let old_cracker_points =
                                                    cracker_lock.points.clone();

                                                client.cracker_count += old_cracker_points;

                                                let old_cracker_points =
                                                    cracker_lock.points.clone();
                                                let old_cracker_pos_x = cracker_lock.x_pos.clone();
                                                let old_cracker_pos_y = cracker_lock.y_pos.clone();

                                                // create a new cracker
                                                *cracker_lock = generate_random_cracker_data();

                                                let got_cracker_msg = GotCrackers {
                                                    action_type:
                                                        OutgoingGameActionType::GotCrackers,
                                                    data: GotCrackersData {
                                                        player_uuid: client.client_id.clone(),
                                                        player_friendly_name: client
                                                            .friendly_name
                                                            .clone(),
                                                        old_cracker_x_position: old_cracker_pos_x,
                                                        old_cracker_y_position: old_cracker_pos_y,
                                                        cracker_point_value: old_cracker_points,
                                                        new_player_score: client.cracker_count,
                                                        new_cracker_x_position: cracker_lock.x_pos,
                                                        new_cracker_y_position: cracker_lock.y_pos,
                                                    },
                                                };

                                                let cracker_msg_string =
                                                    serde_json::ser::to_string(&got_cracker_msg);

                                                match cracker_msg_string {
                                                    Ok(string_response) => {
                                                        let immutable_clients_lock =
                                                            clients.lock().await;

                                                        for (_, tx) in immutable_clients_lock.iter()
                                                        {
                                                            if let Some(current_sender) = &tx.sender
                                                            {
                                                                if &tx.client_id
                                                                    != &client.client_id
                                                                {
                                                                    let _ = &current_sender.send(
                                                                        Ok(Message::text(
                                                                            &string_response,
                                                                        )),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        println!("Couldn't convert ResponseMove struct to a string");
                                                    }
                                                }
                                            }

                                            // tell everyone that someone moved
                                            let rm = SomeoneMovedMsg {
                                                action_type: OutgoingGameActionType::SomeoneMoved,
                                                data: SomeoneMovedData {
                                                    player_uuid: client
                                                        .client_id
                                                        .clone()
                                                        .to_string(),
                                                    player_friendly_name: client
                                                        .friendly_name
                                                        .clone(),
                                                    color: client.color.clone(),
                                                    old_x_position: old_client_x_pos,
                                                    old_y_position: old_client_y_pos,
                                                    new_x_position: client.x_pos,
                                                    new_y_position: client.y_pos,
                                                },
                                            };

                                            let res = serde_json::ser::to_string(&rm).expect(
                                                "Couldn't convert SomeoneMovedMsg struct to a string",
                                            );

                                            sender.send(Ok(Message::text(res))).unwrap()
                                        }
                                    }
                                    IncomingGameActionType::Quack => {
                                        println!("handling quack action type for {:?}!", client_id);

                                        // Don't care about incoming data.

                                        let immutable_clients_lock = clients.lock().await;

                                        println!("got lock {:?}!", client_id);

                                        // Iterate over all clients and send the message
                                        for (_, tx) in immutable_clients_lock.iter() {
                                            let quack_message = SomeoneQuacked {
                                                action_type: OutgoingGameActionType::SomeoneQuacked,
                                                data: QuackData {
                                                    player_uuid: request_originator_client
                                                        .client_id
                                                        .clone(),
                                                    player_friendly_name: request_originator_client
                                                        .friendly_name
                                                        .clone(),
                                                    player_x_position: request_originator_client
                                                        .x_pos,
                                                    player_y_position: request_originator_client
                                                        .y_pos,
                                                    quack_pitch: request_originator_client
                                                        .quack_pitch,
                                                },
                                            };

                                            println!("Checking if client {:?}!", &client_id);

                                            if let Some(current_sender) = &tx.sender {
                                                println!("Sending to {:?}!", &tx.client_id);

                                                // only send it to OTHER connections, not initiator of quack action message
                                                if &tx.client_id != client_id {
                                                    let quack_message_string = serde_json::ser::to_string(&quack_message).expect("Couldn't convert ResponseMove struct to a string");
                                                    current_sender
                                                        .send(Ok(Message::text(
                                                            quack_message_string,
                                                        )))
                                                        .unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(err) => println!(
                                "Couldn't convert incoming action_type to enum: {}, err: {:?}",
                                incoming_request.action_type, err
                            ),
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        sender.send(Ok(Message::text("invalid JSON 😢"))).unwrap();

                        return;
                    }
                };
            }
        }
        None => return,
    }
}
