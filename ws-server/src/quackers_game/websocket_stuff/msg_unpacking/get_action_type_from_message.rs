use std::str::FromStr;

use crate::quackers_game::types::msg_types::{GenericIncomingRequest, IncomingGameActionType};

pub fn get_action_type_from_message(
    json_message: &GenericIncomingRequest,
) -> IncomingGameActionType {
    IncomingGameActionType::from_str(&json_message.action_type).unwrap_or_else(|_err| {
        println!(
            "Did not recognize incoming request action type: {}",
            &json_message.action_type
        );
        IncomingGameActionType::Empty
    })
}
