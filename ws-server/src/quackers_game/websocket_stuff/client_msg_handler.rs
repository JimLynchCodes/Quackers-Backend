use crate::quackers_game::messages::join::receive_submit_name_request::receive_submit_name_action;
use crate::quackers_game::messages::player_move::move_handler::handle_move_action;
use crate::quackers_game::messages::quack::quack_handler::handle_quack_action;
use crate::quackers_game::types::msg_types::{GenericIncomingRequest, IncomingGameActionType};
use crate::quackers_game::websocket_stuff::msg_unpacking::get_action_type_from_message::get_action_type_from_message;
use crate::quackers_game::websocket_stuff::msg_unpacking::unpack_generic_msg::unpack_generic_message;
use crate::{ClientConnections, ClientsGameData, Cracker, Leaderboard};

use warp::ws::Message;

pub async fn client_msg_handler(
    client_id: &str,
    msg: Message,
    client_connections_arc_mutex: &ClientConnections,
    client_data_arc_mutex: &ClientsGameData,
    cracker: &Cracker,
    leaderboard: &Leaderboard,
) {
    println!("received message from {}: {:?}", client_id, msg);

    let json_message: GenericIncomingRequest = unpack_generic_message(msg);

    let action_type = get_action_type_from_message(&json_message);

    println!("action_type is {:?}", action_type);

    match action_type {
        IncomingGameActionType::Join => {
            receive_submit_name_action(
                client_id,
                json_message.clone(),
                &client_connections_arc_mutex,
                &client_data_arc_mutex,
                &cracker,
                &leaderboard
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
                leaderboard
            )
            .await
        }
        IncomingGameActionType::Interact => (),
        IncomingGameActionType::Empty => (),
    }
}
