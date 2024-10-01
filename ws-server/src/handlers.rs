use crate::{quackers_game_logic::client_connection_handler::client_connection, Clients, Cracker, Result};
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws, clients: Clients, cracker: Cracker) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| client_connection(socket, clients, cracker)))
}
