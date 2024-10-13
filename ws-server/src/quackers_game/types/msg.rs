use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenericIncomingRequest {
    pub action_type: String,
    pub data: Value,
}

#[derive(Debug, PartialEq, EnumString, Serialize)]
pub enum IncomingGameActionType {
    #[strum(serialize = "join", serialize = "j")]
    Join,

    #[strum(serialize = "quack", serialize = "q")]
    Quack,

    #[strum(serialize = "move", serialize = "m")]
    Move,

    #[strum(serialize = "interact", serialize = "i")]
    Interact,

    #[strum(serialize = "empty", serialize = "e")]
    Empty, // used as a default in order to ignore invalid inputs without panicing
}

#[derive(Debug, PartialEq, EnumString, Serialize)]
pub enum OutgoingGameActionType {
    #[strum(serialize = "you_joined", serialize = "yj")]
    YouJoined,
    #[strum(serialize = "other_player_joined", serialize = "opj")]
    OtherPlayerJoined,

    #[strum(serialize = "you_quacked", serialize = "yq")]
    YouQuacked,
    #[strum(serialize = "other_player_quacked", serialize = "opq")]
    OtherPlayerQuacked,

    #[strum(serialize = "you_moved", serialize = "ym")]
    YouMoved,
    #[strum(serialize = "other_player_moved", serialize = "opm")]
    OtherPlayerMoved,

    #[strum(serialize = "you_got_crackers", serialize = "ygc")]
    YouGotCrackers,
    #[strum(serialize = "other_player_got_crackers", serialize = "opgc")]
    OtherPlayerGotCrackers,

    #[strum(serialize = "you_died", serialize = "yd")]
    YouGotDied,
    #[strum(serialize = "other_player_died", serialize = "opd")]
    OtherPlayerGotDied,
    
    #[strum(serialize = "user_disconnected", serialize = "ud")]
    UserDisconnected,
    
    #[strum(serialize = "leaderboard_update", serialize = "lu")]
    LeaderboardUpdate,
}
