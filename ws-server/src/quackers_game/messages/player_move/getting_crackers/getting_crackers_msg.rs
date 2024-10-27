use warp::filters::ws::Message;

use crate::quackers_game::types::msg_types::OutgoingGameActionType;

use super::getting_crackers_types::{GotCrackerResponseData, YouGotCrackerMsg};

pub async fn build_other_player_got_cracker_msg(
    got_cracker_response_data: &GotCrackerResponseData,
) -> Message {
    let other_player_got_cracker_message_struct = YouGotCrackerMsg {
        action_type: OutgoingGameActionType::OtherPlayerGotCrackers,
        data: got_cracker_response_data.clone(),
    };

    let other_player_got_cracker_msg_string = serde_json::ser::to_string(
        &other_player_got_cracker_message_struct,
    )
    .unwrap_or_else(|_op| {
        println!("Couldn't convert OtherPlayerGotCracker struct to string");
        "".to_string()
    });

    Message::text(other_player_got_cracker_msg_string)
}

#[cfg(test)]
mod tests {
    use warp::filters::ws::Message;

    use crate::quackers_game::{
        messages::player_move::getting_crackers::getting_crackers_types::{
            GotCrackerResponseData, OtherPlayerGotCrackersMsg,
        },
        types::msg_types::OutgoingGameActionType,
    };

    use super::build_other_player_got_cracker_msg;

    #[tokio::test]
    async fn builds_cracker_msg() {
        let mock_got_cracker_response_data = GotCrackerResponseData::default();

        let result = build_other_player_got_cracker_msg(&mock_got_cracker_response_data).await;

        let expected_struct = OtherPlayerGotCrackersMsg {
            action_type: OutgoingGameActionType::OtherPlayerGotCrackers,
            data: mock_got_cracker_response_data,
        };

        let expected = Message::text(serde_json::to_string(&expected_struct).unwrap());

        assert_eq!(result, expected);
    }
}
