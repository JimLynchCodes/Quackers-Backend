use warp::filters::ws::Message;

use crate::quackers_game::game::game_state::ClientGameData;
use crate::quackers_game::types::msg::OutgoingGameActionType;
use crate::quackers_game::types::player_join_msg::DuckDirection;
use crate::ClientsGameData;

use super::quack_types::{QuackResponseData, YouQuackedMsg};


pub async fn build_you_quacked_msg(
    quacker_client_id: &str,
    quacker_clients_game_data: &ClientsGameData,
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
        leaderboard_position: 0
    };

    let gaurd = quacker_clients_game_data.lock().await;

    let sender_game_data = gaurd.get(quacker_client_id).unwrap_or_else(|| {
        println!("Couldn't find client with id: {}", quacker_client_id);
        &default_game_data
    });

    let quack_message_struct = YouQuackedMsg {
        action_type: OutgoingGameActionType::YouQuacked,
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
