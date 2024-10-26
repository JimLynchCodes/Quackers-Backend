use warp::filters::ws::Message;

use crate::{quackers_game::{game::game_state::ClientGameData, types::msg::OutgoingGameActionType}, ClientsGameData};

use super::quack_types::{OtherQuackedMsg, QuackResponseData};


pub async fn build_other_player_quacked_msg(
    quacker_client_id: &str,
    quacker_clients_game_data: &ClientsGameData,
) -> Message {

    let gaurd = quacker_clients_game_data.lock().await;

    let err_inst = ClientGameData::error_instance();

    let sender_game_data = gaurd.get(quacker_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", quacker_client_id);
        &err_inst
    });

    let quack_message_struct = OtherQuackedMsg {
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
        serde_json::ser::to_string(&quack_message_struct).unwrap_or_else(|_op| {
            println!("Couldn't convert You Quacked struct to string");
            "".to_string()
        });

    Message::text(quack_message_string)
}
