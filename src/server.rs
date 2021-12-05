use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::metrics;

pub type Connections = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub async fn user_connected(ws: WebSocket, connections: Connections) {
    let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new user: {}", id);

    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| eprintln!("websocket send error: {}", e))
                .await;
        }
    });

    connections.write().await.insert(id, tx);

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error (id={}): {}", id, e);
                break;
            }
        };

        user_message(msg, &connections).await;
    }

    eprintln!("user disconnected: {}", id);
    connections.write().await.remove(&id);
}

#[derive(Deserialize)]
struct DistanceRequest {
    string1: String,
    string2: String,
}

#[derive(Serialize, Clone)]
struct DistanceResponse {
    string1: String,
    string2: String,
    hamming_distance: f64,
    levenshtein_distance: f64,
    jaro_distance: f64,
}

impl DistanceResponse {
    fn new(string1: String, string2: String) -> Self {
        let hamming_distance = metrics::hamming_distance(&string1, &string2);
        let levenshtein_distance = metrics::levenshtein_distance(&string1, &string2);
        let jaro_distance = metrics::jaro_distance(&string1, &string2);
        DistanceResponse {
            string1,
            string2,
            hamming_distance,
            levenshtein_distance,
            jaro_distance,
        }
    }
}

async fn user_message(msg: Message, connections: &Connections) {
    if let Ok(msg) = msg.to_str() {
        let distance_request =
            serde_json::from_str::<DistanceRequest>(msg).expect("failed to deserialize request");

        // TODO: calculate the distance
        let distance_response =
            DistanceResponse::new(distance_request.string1, distance_request.string2);

        let response =
            serde_json::to_string(&distance_response).expect("failed to serialize response");

        for (_, tx) in connections.read().await.iter() {
            // Ok - do nothing
            // Err - tunnel was closed and is being removed from the connections dictionary
            let _ = tx.send(Message::text(response.clone()));
        }
    }
}
