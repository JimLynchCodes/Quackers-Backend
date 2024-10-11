use std::str::FromStr;

use crate::quackers_game::msg_handlers::move_handler::handle_move_action;
use crate::quackers_game::msg_handlers::quack_handler::handle_quack_action;
use crate::quackers_game::msg_handlers::submit_name_handler::handle_submit_name_action;
use crate::quackers_game::types::msg::{GenericIncomingRequest, IncomingGameActionType};
use crate::{ClientConnections, ClientsGameData, Cracker};

use serde_json::Value;
use warp::ws::Message;

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
        IncomingGameActionType::Join => {
            handle_submit_name_action(
                client_id,
                json_message.clone(),
                &client_connections_arc_mutex,
                &client_data_arc_mutex,
                &cracker
            )
            .await
        }
        IncomingGameActionType::Quack => {
            handle_quack_action(
                client_id,
                client_connections_arc_mutex,
                client_data_arc_mutex,
            )
            .await;
        }
        IncomingGameActionType::Move => {
            handle_move_action(
                client_id,
                json_message.clone(),
                client_connections_arc_mutex,
                client_data_arc_mutex,
                cracker,
            )
            .await
        }
        IncomingGameActionType::Interact => (),
        IncomingGameActionType::Empty => (),
    }
}

fn get_action_type_from_message(json_message: &GenericIncomingRequest) -> IncomingGameActionType {
    IncomingGameActionType::from_str(&json_message.action_type).unwrap_or_else(|_err| {
        println!(
            "Did not recognize incoming request action type: {}",
            &json_message.action_type
        );
        IncomingGameActionType::Empty
    })
}

fn unpack_message(msg: Message) -> GenericIncomingRequest {
    let message = msg.to_str().unwrap_or_else(|_err| {
        println!("Failed to convert message to string.");
        ""
    });

    println!("message string {:?}", message);

    let json_message: GenericIncomingRequest =
        serde_json::from_str(message).unwrap_or_else(|_err| {
            println!("Failed to convert string message to json.");

            let empty_incoming_request: GenericIncomingRequest = GenericIncomingRequest {
                action_type: "e".to_string(),
                data: Value::String("foo".to_string()),
            };
            empty_incoming_request
        });

    println!("json message: {:?}", json_message);

    json_message
}
