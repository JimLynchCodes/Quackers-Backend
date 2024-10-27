use serde_json::Value;
use warp::filters::ws::Message;

use crate::quackers_game::types::msg_types::GenericIncomingRequest;

pub fn unpack_generic_message(msg: Message) -> GenericIncomingRequest {
    let message = msg.to_str().unwrap_or_else(|_err| {
        println!("Failed to convert message to string.");
        ""
    });

    println!("message string {:?}", message);

    let json_message: GenericIncomingRequest =
        serde_json::from_str(message).unwrap_or_else(|_err| {
            println!("Failed to convert string message to json.");

            let empty_incoming_request: GenericIncomingRequest = GenericIncomingRequest {
                action_type: "e".to_string(),
                data: Value::String("foo".to_string()),
            };
            empty_incoming_request
        });

    println!("json message: {:?}", json_message);

    json_message
}
