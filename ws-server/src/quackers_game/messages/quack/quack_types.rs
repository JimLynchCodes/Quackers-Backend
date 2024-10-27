use serde::Serialize;

use crate::quackers_game::types::msg_types::OutgoingGameActionType;

#[derive(Debug, Serialize)]
pub struct QuackRequestData {}

#[derive(Debug, Serialize)]
pub struct QuackResponseData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub player_x_position: f32,
    pub player_y_position: f32,
    pub quack_pitch: f32,
}

#[derive(Debug, Serialize)]
pub struct YouQuackedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: QuackResponseData,
}

#[derive(Debug, Serialize)]
pub struct OtherQuackedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: QuackResponseData,
}