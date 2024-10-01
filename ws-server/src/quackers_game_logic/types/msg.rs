use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::EnumString;

#[derive(Debug, Deserialize)]
pub struct GenericIncomingRequest {
    pub action_type: String,
    pub data: Value,
}

#[derive(Debug, PartialEq, EnumString, Serialize)]
pub enum IncomingGameActionType {
    #[strum(serialize = "quack", serialize = "q")]
    Quack,

    #[strum(serialize = "player_move", serialize = "pm")]
    PlayerMove,
}

#[derive(Debug, PartialEq, EnumString, Serialize)]
pub enum OutgoingGameActionType {
    #[strum(serialize = "someone_quacked", serialize = "sq")]
    SomeoneQuacked,

    #[strum(serialize = "someone_moved", serialize = "sm")]
    SomeoneMoved,

    #[strum(serialize = "got_crackers", serialize = "gc")]
    GotCrackers,
}
