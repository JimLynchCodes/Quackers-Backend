use tungstenite::{connect, Message};
use serde::Serialize;

fn main() {
    // env_logger::init();

    let (mut socket, response) = connect("ws://127.0.0.1:8000/ws").expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    let fake_join_request_msg = JoinRequestMsg {
        action_type: "join".to_string(),
        data: JoinedRequestData {
            friendly_name: "foo".to_string()
        }
    };

    let stringified_join_request = serde_json::ser::to_string(&fake_join_request_msg)
        .unwrap_or_else(|_op| {
            println!("Couldn't convert fake_join_request_msg struct to string");
            "".to_string()
        });

    socket.send(Message::Text(stringified_join_request)).unwrap();
    
    let mut received_messages = vec![]; 

    
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {msg}");
        
        received_messages.push(msg);
    }
    
    // Doesn't actually get down to here
    assert_eq!(true, false);

    socket.close(None);
}

#[derive(Serialize)]
struct JoinedRequestData {
    friendly_name: String
}

#[derive(Serialize)]
struct JoinRequestMsg {
    action_type: String,
    data: JoinedRequestData
}