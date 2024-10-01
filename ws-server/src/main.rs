use std::{collections::HashMap, convert::Infallible, sync::Arc};
use quackers_game_logic::{cracker_creator::generate_random_cracker_data, types::game_state::{Client, CrackerData}};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

mod handlers;
mod quackers_game_logic;

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Cracker = Arc<Mutex<CrackerData>>;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let cracker: Cracker = Arc::new(Mutex::new(generate_random_cracker_data()));

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_cracker(cracker.clone()))
        .and_then(handlers::ws_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server on ws://127.0.0.1:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_cracker(cracker: Cracker) -> impl Filter<Extract = (Cracker,), Error = Infallible> + Clone {
    warp::any().map(move || cracker.clone())
}
