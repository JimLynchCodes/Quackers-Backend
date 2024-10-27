mod quackers_game;
mod ws_handlers;

use std::{collections::HashMap, convert::Infallible, sync::Arc};
use quackers_game::game::{cracker_creator::generate_random_cracker_data, game_state::{ClientConnection, ClientGameData, CrackerData, LeaderboardData}};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};
use ws_handlers::ws_handler;

type ClientConnections = Arc<Mutex<HashMap<String, ClientConnection>>>;
type ClientsGameData = Arc<Mutex<HashMap<String, ClientGameData>>>;
type Leaderboard = Arc<Mutex<LeaderboardData>>;
type Cracker = Arc<Mutex<CrackerData>>;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let clients: ClientConnections = Arc::new(Mutex::new(HashMap::new()));
    let clients_game_data: ClientsGameData = Arc::new(Mutex::new(HashMap::new()));
    let cracker: Cracker = Arc::new(Mutex::new(generate_random_cracker_data()));
    let leaderboard: Leaderboard = Arc::new(Mutex::new(LeaderboardData::new_leaderboard()));

    println!("Configuring websocket route");

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_client_connections(clients.clone()))
        .and(with_clients_game_data(clients_game_data.clone()))
        .and(with_cracker(cracker.clone()))
        .and(with_leaderboard(leaderboard.clone()))
        .and_then(ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server on ws://127.0.0.1:8000/ws");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_client_connections(clients: ClientConnections) -> impl Filter<Extract = (ClientConnections,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_clients_game_data(clients: ClientsGameData) -> impl Filter<Extract = (ClientsGameData,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_cracker(cracker: Cracker) -> impl Filter<Extract = (Cracker,), Error = Infallible> + Clone {
    warp::any().map(move || cracker.clone())
}

fn with_leaderboard(leaderboard: Leaderboard) -> impl Filter<Extract = (Leaderboard,), Error = Infallible> + Clone {
    warp::any().map(move || leaderboard.clone())
}
