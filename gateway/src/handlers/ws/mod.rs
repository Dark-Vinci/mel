use {
    crate::handlers::AppState,
    axum::{
        extract::{
            ws::{Message, WebSocket},
            State, WebSocketUpgrade,
        },
        response::IntoResponse,
        Router,
    },
    futures_util::{SinkExt, StreamExt},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    tokio::sync::broadcast,
};

// #[derive(Debug, Clone)]
pub struct Hub {
    users: Mutex<HashMap<String, String>>,
    tx: broadcast::Sender<UserMessage>,
}

impl Hub {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(32);

        Self {
            tx,
            users: Mutex::new(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone)]
enum Actions {
    Scream,
    Chat,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserMessage {
    user_id: String,
    to_user: String,
    content: String,
    action: Actions,
}

pub fn handler(state: &AppState) -> Router {
    Router::new().route("/", get(ws_handler)).with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(app): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, app))
}

async fn handle_socket(mut socket: WebSocket, hub: AppState) {
    let (mut sender, mut receiver) = socket.split();

    let username = "String::new()";

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(msg) = message {
            let ser_msg = serde_json::from_str::<UserMessage>(&msg).unwrap();

            let u_cl = ser_msg.clone();

            let username = ser_msg.user_id;
            let content = ser_msg.to_user;
            hub.ws_hub.users.lock().unwrap().insert(username, content);

            if ser_msg.user_id.is_empty() {
                break;
            } else {
                let _ = sender.send(Message::from(u_cl)).await;
                return;
            }
        }
    }

    let mut rx = hub.ws_hub.tx.subscribe();

    // send messages to client
    let mut send_pump = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg.to_user == username {
                if sender.send(Message::from(msg)).await.is_err() {
                    break;
                }
            }
        }
    });

    let tx = hub.ws_hub.tx.clone();

    // read message from user
    let mut read_pump = tokio::spawn(async move {
        while let Ok(msg) = receiver.next().await {
            if let Message::Text(msg) = msg {
                let ss = serde_json::from_str::<UserMessage>(&msg).unwrap();

                match ss.action {
                    Actions::Scream => {
                        println!("Scream");
                        let _ = tx.send(ss).await;
                    },

                    Actions::Chat => {
                        println!("Chat");
                    },
                }
            }
        }
    });

    tokio::select! {
        _ = &mut read_pump => send_pump.abort(),
        _ = &mut send_pump => read_pump.abort(),
    }

    hub.ws_hub.users.lock().unwrap().remove(&username);
}
