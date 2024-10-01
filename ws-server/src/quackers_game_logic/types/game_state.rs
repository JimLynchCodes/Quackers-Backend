use tokio::sync::mpsc;
use warp::filters::ws::Message;

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
    pub points: u64,
    pub x_pos: u64,
    pub y_pos: u64,
    pub radius: u64,
}
