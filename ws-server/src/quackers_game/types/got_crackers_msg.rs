use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::msg::OutgoingGameActionType;

#[derive(Debug, Serialize)]
pub struct GotCrackersResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub old_cracker_x_position: u64,
    pub old_cracker_y_position: u64,
    pub cracker_point_value: u64,
    pub new_player_score: u64,
    pub new_cracker_x_position: u64,
    pub new_cracker_y_position: u64,
}

#[derive(Debug, Serialize)]
pub struct YouGotCrackersMsg {
    pub action_type: OutgoingGameActionType,
    pub data: GotCrackersResponseData,
}

#[derive(Debug, Serialize)]
pub struct OtherPlayerGotCrackersMsg {
    pub action_type: OutgoingGameActionType,
    pub data: GotCrackersResponseData,
}
