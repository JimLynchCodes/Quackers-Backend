use super::msg_types::OutgoingGameActionType;
use serde::Serialize;

// Player sends a "friendly name" and then other players can see
// Note: currently name is generated randomly by server and this friendly_name is ignored

#[derive(Debug, Serialize)]
pub struct LeaderboardUpdateData {
    pub your_points: u64,
    pub your_leaderboard_place: u64,

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

#[derive(Debug, Serialize)]
pub struct LeaderboardUpdateMsg {
    pub action_type: OutgoingGameActionType,
    pub data: LeaderboardUpdateData,
}

