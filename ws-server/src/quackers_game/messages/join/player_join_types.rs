
use serde::{Deserialize, Serialize};

use crate::quackers_game::{game::game_state::DuckDirection, messages::msg_types::OutgoingGameActionType};

// Player sends a "friendly name" and then other players can see
// Note: currently name is generated randomly by server and this friendly_name is ignored

#[derive(Debug, Deserialize)]
pub struct JoinRequestData {
    pub friendly_name: String,
}

#[derive(Debug, Serialize)]
pub struct OtherPlayerData {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub x_position: f32,
    pub y_position: f32,

    pub direction_facing: DuckDirection,
}

#[derive(Debug, Serialize)]
pub struct NewJoinerDataWithAllPlayers {
    pub player_uuid: String,
    pub player_friendly_name: String,
    pub color: String,
    pub x_position: f32,
    pub y_position: f32,
    pub cracker_x: f32,
    pub cracker_y: f32,
    pub cracker_points: u64,

    pub player_points: u64,

    pub all_other_players: Vec<OtherPlayerData>,
}

#[derive(Debug, Serialize)]
pub struct YouJoinedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: NewJoinerDataWithAllPlayers,
}

#[derive(Debug, Serialize)]
pub struct OtherPlayerJoinedMsg {
    pub action_type: OutgoingGameActionType,
    pub data: OtherPlayerData,
}
