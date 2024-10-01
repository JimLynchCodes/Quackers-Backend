use crate::{Client, Clients, Cracker};
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use std::convert::TryInto;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use strum_macros::EnumString;

const X_DEFAULT_START_POSTION: u64 = 10;
const Y_DEFAULT_START_POSTION: u64 = 10;

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
}

pub async fn client_connection(ws: WebSocket, clients: Clients, cracker: Cracker) {
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
        radius: 20,
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
        client_msg(&uuid, msg, &clients, &cracker).await;
    }

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}

async fn client_msg(client_id: &str, msg: Message, clients: &Clients, cracker: &Cracker) {
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
                struct PlayerMoveRequest {
                    action_type: String,
                    data: Value,
                }

                #[derive(Debug, Serialize)]
                struct ResponseMove {
                    action_type: String,
                    data: String,
                }

                let json_result: Result<PlayerMoveRequest, _> = serde_json::from_str(message);

                match json_result {
                    Ok(v) => {
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

                                            // if let Some(cracker) = cracker_lock. {
                                            //     // check if duck is close to crackers
                                            //     // good old pythagorean theorem!
                                            let distance: f64 = ((client.x_pos - cracker.x_pos)
                                                .pow(2)
                                                + (client.y_pos - crackers.y_pos).pow(2))
                                            .sqrt();

                                            if distance < (client.radius + crackers.radius) {
                                                let cracker_lock = cracker.lock().await;

                                                client.cracker_count += cracker_lock.points;

                                                // TODO create a new 

                                                // award player!

                                                // TODO send message

                                            }

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
