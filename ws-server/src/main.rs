use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod handlers;
mod ws;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,

    pub x_pos: u64,
    pub y_pos: u64,
    pub radius: u64,

    pub friendly_name: String,
    pub color: String,
    pub quack_pitch: f64,

    pub cracker_count: u64,
}

pub struct CrackerData {
    points: u64,
    x_pos: u64,
    y_pos: u64,
    radius: u64,
}

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Cracker = Arc<Mutex<CrackerData>>;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let cracker: Cracker = Arc::new(Mutex::new(CrackerData {
        points: 10,
        x_pos: 1,
        y_pos: 1,
        radius: 2,
    }));

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
