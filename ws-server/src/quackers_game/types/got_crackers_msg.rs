use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::msg::OutgoingGameActionType;

#[derive(Debug, Serialize, Clone)]
pub struct GotCrackerResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub old_cracker_x_position: f32,
    pub old_cracker_y_position: f32,
    pub cracker_point_value: u64,
    pub new_player_score: u64,
    pub new_cracker_x_position: f32,
    pub new_cracker_y_position: f32,
}

#[derive(Debug, Serialize)]
pub struct YouGotCrackerMsg {
    pub action_type: OutgoingGameActionType,
    pub data: GotCrackerResponseData,
}

#[derive(Debug, Serialize)]
pub struct OtherPlayerGotCrackersMsg {
    pub action_type: OutgoingGameActionType,
    pub data: GotCrackerResponseData,
}
