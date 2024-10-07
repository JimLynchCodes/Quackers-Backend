use crate::quackers_game::cracker_creator::generate_random_cracker_data;
use crate::quackers_game::msg_handlers::move_handler::handle_move_action;
use crate::quackers_game::msg_handlers::quack_handler::handle_quack_action;
use crate::quackers_game::msg_handlers::submit_name_handler::handle_submit_name_action;
use crate::quackers_game::types::defaults::{MAX_X_POS, MAX_Y_POS, MIN_X_POS, MIN_Y_POS};
use crate::quackers_game::types::msg::{
    GenericIncomingRequest, IncomingGameActionType, OutgoingGameActionType,
};
use crate::quackers_game::types::player_move_msg::MoveRequestData;
use crate::{ClientConnections, ClientsGameData, Cracker};

use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::convert::TryInto;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;

use super::types::game_state::{ClientConnection, ClientGameData};
use super::types::quack_msg::{QuackResponseData, YouQuackedMsg};
// use super::types::*;

pub async fn client_msg(
    client_id: &str,
    msg: Message,
    client_connections_arc_mutex: &ClientConnections,
    client_data_arc_mutex: &ClientsGameData,
    cracker: &Cracker,
) {
    println!("received message from {}: {:?}", client_id, msg);
    
    let json_message: GenericIncomingRequest = unpack_message(msg);
    
    let action_type = get_action_type_from_message(&json_message);
    
    println!("action_type is {:?}", action_type);

    match action_type {
        IncomingGameActionType::Join => handle_submit_name_action(
            client_id,
            json_message.clone(),
            &client_connections_arc_mutex,
            &client_data_arc_mutex,
        ).await,
        IncomingGameActionType::Quack => {
            handle_quack_action(
                client_id,
                client_connections_arc_mutex,
                client_data_arc_mutex,
            )
            .await;
        }
        IncomingGameActionType::Move => handle_move_action(client_id, client_connections_arc_mutex, client_data_arc_mutex, cracker).await,
        IncomingGameActionType::Interact => (),
        IncomingGameActionType::Empty => (),
    }
}

// fn handle_submit_name_action(
//     client_id: &str,
//     json_message: GenericIncomingRequest,
//     client_connections_arc_mutex: &ClientConnections,
//     client_data_arc_mutex: &ClientsGameData,
// ) {
//     let incoming_data_json: Result<MoveRequestData, _> = serde_json::from_value(json_message.data);
// }

fn get_action_type_from_message(json_message: &GenericIncomingRequest) -> IncomingGameActionType {
    IncomingGameActionType::from_str(&json_message.action_type).unwrap_or_else(|err| {
        println!(
            "Did not recognize incoming request action type: {}",
            &json_message.action_type
        );
        IncomingGameActionType::Empty
    })
}

fn unpack_message(msg: Message) -> GenericIncomingRequest {
    let message = msg.to_str().unwrap_or_else(|err| {
        println!("Failed to convert message to string.");
        ""
    });

    println!("message string {:?}", message);

    let json_message: GenericIncomingRequest =
        serde_json::from_str(message).unwrap_or_else(|err| {
            println!("Failed to convert string message to json.");

            let EMPTY_INCOMING_REQUEST: GenericIncomingRequest = GenericIncomingRequest {
                action_type: "e".to_string(),
                data: Value::String("foo".to_string()),
            };
            EMPTY_INCOMING_REQUEST
        });

    println!("json message: {:?}", json_message);

    json_message
}
