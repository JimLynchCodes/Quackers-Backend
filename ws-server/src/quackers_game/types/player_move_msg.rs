use serde::{Deserialize, Serialize};
use super::msg::OutgoingGameActionType;

#[derive(Debug, Deserialize)]
pub struct MoveRequestData {
    pub x_direction: u64,
    pub y_direction: u64,
}

#[derive(Debug, Serialize)]
pub struct MoveResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub old_x_position: u64,
    pub old_y_position: u64,
    pub new_x_position: u64,
    pub new_y_position: u64,
}

#[derive(Debug, Serialize)]
pub struct YouMovedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: MoveResponseData,
}

#[derive(Debug, Serialize)]
pub struct OtherMovedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: MoveResponseData,
}



