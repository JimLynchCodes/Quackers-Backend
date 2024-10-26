use serde::Serialize;
use tokio::sync::mpsc;
use warp::filters::ws::Message;

use crate::quackers_game::types::player_join_msg::DuckDirection;

#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClientGameData {
    pub client_id: String,
    pub x_pos: f32,
    pub y_pos: f32,
    pub direction_facing: DuckDirection,
    pub radius: u64,

    pub friendly_name: String,
    pub color: String,
    pub quack_pitch: f32,

    pub cracker_count: u64,
    pub leaderboard_position: u64,
}

impl ClientGameData {

    pub fn error_instance() -> Self {
        Self {
            client_id: "error".to_string(),
            x_pos: 0.,
            y_pos: 0.,
            direction_facing: DuckDirection::Right,
            radius: 0,
            friendly_name: "error".to_string(),
            color: "error".to_string(),
            quack_pitch: 0.,
            cracker_count: 0,
            leaderboard_position: 0,
        }
    }
}

pub struct CrackerData {
    pub points: u64,
    pub x_pos: f32,
    pub y_pos: f32,
    pub radius: u64,
}

#[derive(Default)]
pub struct LeaderboardData {
    pub leaderboard_name_1st_place: String,
    pub leaderboard_name_2nd_place: String,
    pub leaderboard_name_3rd_place: String,
    pub leaderboard_name_4th_place: String,
    pub leaderboard_name_5th_place: String,

    pub leaderboard_score_1st_place: u64,
    pub leaderboard_score_2nd_place: u64,
    pub leaderboard_score_3rd_place: u64,
    pub leaderboard_score_4th_place: u64,
    pub leaderboard_score_5th_place: u64,
}

impl LeaderboardData {
    pub fn new_leaderboard() -> Self {
        LeaderboardData {
            leaderboard_name_1st_place: "--".to_string(),
            leaderboard_name_2nd_place: "--".to_string(),
            leaderboard_name_3rd_place: "--".to_string(),
            leaderboard_name_4th_place: "--".to_string(),
            leaderboard_name_5th_place: "--".to_string(),
            leaderboard_score_1st_place: 0,
            leaderboard_score_2nd_place: 0,
            leaderboard_score_3rd_place: 0,
            leaderboard_score_4th_place: 0,
            leaderboard_score_5th_place: 0,
        }
    }
}
