use serde::{Deserialize, Serialize};
use super::msg::OutgoingGameActionType;

// Player sends a "friendly name" and then other players can see

#[derive(Debug, Deserialize)]
pub struct JoinRequestData {
    pub friendly_name: String,
}

#[derive(Debug, Serialize)]
pub struct NewJoinerData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub x_position: u64,
    pub y_position: u64,
}

#[derive(Debug, Serialize)]
pub struct YouJoinedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: NewJoinerData,
}

#[derive(Debug, Serialize)]
pub struct OtherPlayerJoinedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: NewJoinerData,
}


