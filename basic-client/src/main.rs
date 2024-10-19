use tungstenite::connect;

const SERVER_URL: &str = "ws://0.0.0.0:8000";

fn main() {
    let (mut socket, _response) = connect(SERVER_URL).expect("Can't connect");

    println!("Connected to the the server!");

    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received websocket: {msg}");
    }
}
