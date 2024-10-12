use serde::Serialize;

use super::msg::OutgoingGameActionType;

#[derive(Debug, Serialize, Clone)]
pub struct UserDisconnectedData {
    pub disconnected_player_uuid: String,
}

#[derive(Debug, Serialize)]
pub struct UserDisconnectedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: UserDisconnectedData,
}
