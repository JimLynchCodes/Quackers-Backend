use crate::{
    quackers_game::websocket_stuff::client_connection_handler::client_connection_handler, ClientConnections, ClientsGameData, Cracker, Leaderboard, Result
};
use warp::Reply;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    client_connections: ClientConnections,
    clients_game_data: ClientsGameData,
    cracker: Cracker,
    leaderboard: Leaderboard,
) -> Result<impl Reply> {
    Ok(ws.on_upgrade(move |socket| {
        client_connection_handler(socket, client_connections, clients_game_data, cracker, leaderboard)
    }))
}
