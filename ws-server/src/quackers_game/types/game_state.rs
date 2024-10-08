use tokio::sync::mpsc;
use warp::filters::ws::Message;

#[derive(Debug, Clone)]
pub struct ClientConnection {
    pub client_id: String, // used as the key in has map
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Debug, Clone)]
pub struct ClientGameData {
    pub client_id: String, // used as the key in has map
    pub x_pos: f32,
    pub y_pos: f32,
    pub radius: u64,

    pub friendly_name: String,
    pub color: String,
    pub quack_pitch: f32,

    pub cracker_count: u64,
}

pub struct CrackerData {
    pub points: u64,
    pub x_pos: f32,
    pub y_pos: f32,
    pub radius: u64,
}
