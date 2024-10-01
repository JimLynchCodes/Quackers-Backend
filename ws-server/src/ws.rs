use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use std::convert::TryInto;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;

const ACTION_TYPE_MOVE: &str = "player_move";
const ACTION_TYPE_QUACK: &str = "quack";
const ACTION_TYPE_INTERACT: &str = "interact";

const X_DEFAULT_START_POSTION: u64 = 10;
const Y_DEFAULT_START_POSTION: u64 = 10;

// client_id -> playergameData

#[derive(Debug, PartialEq, EnumString)]
enum GameActionType {
    Red,

    #[strum(serialize = "player_move", serialize = "pm")]
    PlayerMove {
        x_direction: usize,
        y_direction: usize,
    },

    #[strum(serialize = "quack", serialize = "q")]
    Quack,
    // We can match on multiple different patterns.
    // #[strum(serialize = "blue", serialize = "b")]
    // Move(usize),

    // // Notice that we can disable certain variants from being found
    // #[strum(disabled)]
    // Yellow,

    // // We can make the comparison case insensitive (however Unicode is not supported at the moment)
    // #[strum(ascii_case_insensitive)]
    // Black,
}

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

        friendly_name: "[NO_NAME]".to_string(),
        color: "red".to_string(),
        quack_pitch: 1.0,
        x_pos: X_DEFAULT_START_POSTION,
        y_pos: Y_DEFAULT_START_POSTION,
        cracker_count: 0,
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
                    data: Value,
                }

                #[derive(Debug, Serialize)]
                struct ResponseMove {
                    action_type: String,
                    data: String,
                }

                // enum ActionTypes {
                //     move = "move"
                // }

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
                        // sender.send(Ok(Message::text("yeahh"))).unwrap();
                        println!("got action of type: {}", v.action_type);

                        match GameActionType::from_str(&v.action_type) {
                            Ok(action_type) => {
                                match action_type {
                                    GameActionType::Red => {
                                        println!("handling action type: Red!");
                                    }
                                    GameActionType::PlayerMove {
                                        x_direction,
                                        y_direction,
                                    } => {
                                        let mut clients_lock = clients.lock().await;
                                        if let Some(client) = clients_lock.get_mut(client_id) {
                                            let x_pos_int: u64 = x_direction
                                                .try_into()
                                                .expect("Converting x_pos failed");
                                            let y_pos_int: u64 = y_direction
                                                .try_into()
                                                .expect("Converting y_pos failed");

                                            client.x_pos += x_pos_int;
                                            client.y_pos += y_pos_int;

                                            // TODO - check if duck is close to crackers

                                            // TODO
                                            // messsage errbody the good lord's news. wait, wut 

                                        }

                                        let rm = ResponseMove {
                                            action_type: "response_stuff".to_string(),
                                            data: "yerrr".to_string(),
                                        };

                                        let res = serde_json::ser::to_string(&rm);

                                        match res {
                                            Ok(string_response) => sender
                                                .send(Ok(Message::text(string_response)))
                                                .unwrap(),
                                            _ => {
                                                println!("Couldn't convert ResponseMove struct to a string")
                                            }
                                        }
                                    }
                                    GameActionType::Quack => {
                                        println!("handling quack action type for {:?}!", client_id);

                                        // Tell all players you quacked
                                        // match res {

                                        println!("mapping over clients");

                                        match sender.send(Ok(Message::text(
                                            format!("you quacked {}", client_id).to_string(),
                                        ))) {
                                            Ok(s) => {
                                                println!("Sent quack response")
                                            }
                                            Err(err) => println!("Failed to response quack"),
                                        }

                                        // Iterate over all clients and send the message
                                        for (_, tx) in locked.iter() {
                                            // let _ = tx.send(Ok(msg.clone()));
                                            let msg = Message::text("foo".to_string());

                                            let _ = tx.sender;

                                            if let Some(current_sender) = &tx.sender {
                                                // only send it to OTHER connections, not initiator of quack action message
                                                if &tx.client_id != client_id {
                                                    let _ = current_sender.send(Ok(msg));
                                                }
                                            }
                                        }
                                    } // let _ = clients.lock().map(|client| {
                                      //     println!("client: {:?}", client);

                                      // });

                                      // for (_, tx) in clients.lock().unwrap().iter() {
                                      //     let _ = tx.send(Ok(msg.clone()));
                                      // }

                                      // Locking the clients map asynchronously
                                      // let clients_guard = clients.lock().await;

                                      // println!("length: {:?}", clients_guard.iter().count());

                                      // // Iterate over all clients and send the message

                                      // for (something, client) in clients_guard.iter() {

                                      //     println!("mapping over client something: {:?}", something);
                                      //     println!("mapping over client client: {:?}", client);

                                      //     // Assuming `client` has a sender that can send messages
                                      //     if let Some(sender) = &client.sender {
                                      //         let _  = sender.send(Ok(Message::text(format!("hey this duck over here quacked! {:?}", "foo").to_string())));
                                      //     }
                                }
                            }
                            Err(err) => println!(
                                "Couldn't convert incoming action_type to enum: {}, err: {:?}",
                                v.action_type, err
                            ),
                        }
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
