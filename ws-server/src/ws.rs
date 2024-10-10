use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::{de::value::Error, Deserialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);

    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));

    let uuid = Uuid::new_v4().simple().to_string();

    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };

    clients.lock().await.insert(uuid.clone(), new_client);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                println!("error receiving message for id {}): {}", uuid.clone(), e);
                break;
            }
        };
        client_msg(&uuid, msg, &clients).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", client_id, msg);

    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    let locked = clients.lock().await;
    match locked.get(client_id) {
        Some(v) => {
            if let Some(sender) = &v.sender {
                println!("Got message from a sender! {:?}", message);
                // let _ = sender.send(Ok(Message::text("pong")));

                #[derive(Debug, Deserialize)]
                struct L0 {
                    action_type: String,
                }

                // let msg_json: Result<Value, _> = serde_json::from_str(message);

                // println!("Unpacked msg string! {:?}", msg_json);

                // let message_json: Value = match msg_json {
                //     Ok(v) => v,
                //     Err(_) => return,
                //     // Err(_) => sender.send(Ok(Message::text("invalid JSON 😢"))),
                // };

                // let message_str: Value = match msg_json {
                //     Ok(v) => v,
                //     Err(_) => return,
                // };

                "{\n    \"action_type\": “bar”\n}\n"
                "\"{\\n    \\\"action_type\\\": “bar”\\n}\\n\""
                let json_result: Result<L0, _> = serde_json::from_str(message);

                // if let Ok(foo) = json_result {

                //     println!("got it!");
                //     println!("{:?}", foo);
                // }

                
                // if let (err) = json_result {

                //     println!("ERRRRR");
                //     println!("{:?}", err);
                // }

                match json_result {
                    Ok(v) => {
                        // println!("Unpacked msg json! {:?}", v);
                        // valid json, try to read actionType

                        // println!("actionType is: {}", v.action_type);
                        sender.send(Ok(Message::text("yeahh"))).unwrap();
                    }
                    // Err(_) => return,
                    Err(err) => {

                        println!("{:?}", err);
                        sender.send(Ok(Message::text("invalid JSON 😢"))).unwrap();
                        // (|msg| {
                            // println!("Error sending {} to {}")
                        // });
                        return;
                    }
                };

            }
        }
        None => return,
    }

    // let msg_json = serde_json::from_str(message);

    // let message_json = match msg_json {
    //     Ok(v) => v,
    //     Err(_) => sender.send(Ok(Message::text("invalid JSON 😢"))),
    // };

    // if message == "ping" || message == "ping\n" {
    //     let locked = clients.lock().await;
    //     match locked.get(client_id) {
    //         Some(v) => {
    //             if let Some(sender) = &v.sender {
    //                 println!("sending pong");
    //                 let _ = sender.send(Ok(Message::text("pong")));
    //             }
    //         }
    //         None => return,
    //     }
    //     return;
    // };
}
