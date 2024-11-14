use warp::filters::ws::Message;

use crate::quackers_game::messages::msg_types::OutgoingGameActionType;

use super::user_disconnected_types::{UserDisconnectedData, UserDisconnectedMsg};

pub fn build_user_disconnected_msg(uuid: &str) -> Message {
    let user_disconnected_message_struct = UserDisconnectedMsg {
        action_type: OutgoingGameActionType::UserDisconnected,
        data: UserDisconnectedData {
            disconnected_player_uuid: uuid.to_string(),
        },
    };

    let user_disconnected_msg_string =
        serde_json::ser::to_string(&user_disconnected_message_struct).unwrap_or_else(|_op| {
            println!("Couldn't convert UserDisconnected struct to string");
            "".to_string()
        });

    Message::text(user_disconnected_msg_string)
}
