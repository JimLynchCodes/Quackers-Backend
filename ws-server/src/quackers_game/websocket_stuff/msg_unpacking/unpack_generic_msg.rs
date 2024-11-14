use warp::filters::ws::Message;

use crate::quackers_game::messages::msg_types::GenericIncomingRequest;

pub fn unpack_generic_message(msg: Message) -> GenericIncomingRequest {
    let message = msg.to_str().unwrap_or_else(|_err| {
        println!("Failed to convert message to string.");
        ""
    });

    println!("message string {:?}", message);

    let json_message: GenericIncomingRequest =
        serde_json::from_str(message).unwrap_or_else(|_err| {
            println!("Failed to convert string message to json.");

            GenericIncomingRequest::default()

            // let empty_incoming_request: GenericIncomingRequest = GenericIncomingRequest {
            //     action_type: "e".to_string(),
            //     data: Value::String("foo".to_string()),
            // };
            // empty_incoming_request
        });

    println!("json message: {:?}", json_message);

    json_message
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use warp::filters::ws::Message;

    use crate::quackers_game::messages::msg_types::GenericIncomingRequest;

    use super::unpack_generic_message;

    #[test]
    fn non_string_input_returns_default() {

        let fake_text = "invalid string";
        let fake_message = Message::text(fake_text);
        
        let result= unpack_generic_message(fake_message);

        assert_eq!(result, GenericIncomingRequest::default());
    }

    #[test]
    fn non_deserializable_input_returns_default() {

        let fake_text = "invalid string";
        let fake_message = Message::text(fake_text);
        
        let result= unpack_generic_message(fake_message);

        assert_eq!(result, GenericIncomingRequest::default());
    }

    #[test]
    fn unpacks_generic_message() {

        let mut fake_input_request = GenericIncomingRequest::default();

        fake_input_request.action_type = "foo".to_string();
        fake_input_request.data = Value::String("bar".to_string());

        let fake_input_string = serde_json::to_string(&fake_input_request).unwrap();

        let fake_input_message = Message::text(fake_input_string);
        
        let result= unpack_generic_message(fake_input_message);

        assert_eq!(result, fake_input_request);
    }
}