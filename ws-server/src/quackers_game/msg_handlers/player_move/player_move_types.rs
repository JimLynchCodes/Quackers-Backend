use serde::{Deserialize, Serialize};

use crate::quackers_game::types::msg::OutgoingGameActionType;

#[derive(Debug, Deserialize)]
pub struct MoveRequestData {
    pub x_direction: f32,
    pub y_direction: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct MoveResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub old_x_position: f32,
    pub old_y_position: f32,
    pub new_x_position: f32,
    pub new_y_position: f32,
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
