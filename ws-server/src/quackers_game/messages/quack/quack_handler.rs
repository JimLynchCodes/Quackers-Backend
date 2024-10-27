use crate::{
    ClientConnections, ClientsGameData,
};

use super::{other_player_quacked_msg::build_other_player_quacked_msg, you_quacked_msg::build_you_quacked_msg};

pub async fn handle_quack_action(
    sender_client_id: &str,
    client_connections_arc_mutex: &ClientConnections,
    clients_game_data_arc_mutex: &ClientsGameData,
) {
    // No need to unpack the request data for quack

    // Send quack message to connected clients
    for (_, tx) in client_connections_arc_mutex.lock().await.iter() {
        if &tx.client_id == sender_client_id {
            let you_quacked_msg =
                build_you_quacked_msg(sender_client_id, &clients_game_data_arc_mutex).await;

            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(you_quacked_msg))
                .unwrap();
        } else {
            let other_player_quacked_msg =
                build_other_player_quacked_msg(sender_client_id, &clients_game_data_arc_mutex)
                    .await;
            tx.sender
                .as_ref()
                .unwrap()
                .send(Ok(other_player_quacked_msg))
                .unwrap();
        }
    }
}
