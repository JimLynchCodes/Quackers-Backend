use serde::Serialize;

use super::msg::OutgoingGameActionType;

#[derive(Debug, Serialize)]
pub struct QuackData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub player_x_position: u64,
    pub player_y_position: u64,
    pub quack_pitch: f64,
}

#[derive(Debug, Serialize)]
pub struct SomeoneQuacked {
    pub action_type: OutgoingGameActionType,
    pub data: QuackData,
}
