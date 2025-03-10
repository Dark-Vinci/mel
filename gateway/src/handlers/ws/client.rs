use {
    axum::extract::ws::{Message, WebSocket},
    futures_util::{
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    },
    serde::{Deserialize, Serialize},
    tokio::sync::{broadcast, mpsc},
    tracing::info,
    uuid::Uuid,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageType {
    content: String,
    to_user: String,
}

#[derive(Debug)]
pub struct Client {
    pub user_id: Uuid,
    pub sender: SplitSink<WebSocket, Message>,
    pub receiver: SplitStream<WebSocket>,
    pub hub_listener: mpsc::Sender<MessageType>, // send to hub
    pub hub_sender: broadcast::Receiver<MessageType>,
}

impl Client {
    pub fn new(
        socket: WebSocket,
        id: Uuid,
        hub_listener: mpsc::Sender<MessageType>,
        hub_sender: broadcast::Receiver<MessageType>,
    ) -> Self {
        let (send, recv) = socket.split();

        Self {
            hub_listener,
            hub_sender,
            sender: send,
            receiver: recv,
            user_id: id,
        }
    }

    // this would be called by the hub
    // read from client, the will be sent to hub to be broadcasted
    pub async fn pumper(&mut self) {
        loop {
            tokio::select! {
                res = self.hub_sender.recv() => {
                    match res {
                        Ok(msg) => {
                            if msg.to_user == self.user_id.to_string() {
                                let msg = serde_json::to_string(&msg).unwrap(); // todo; handle graciously

                                self.sender.send(Message::Text(msg.into())).await.unwrap();
                            }
                        }

                        Err(e) => {
                            tracing::debug!("client disconnected abruptly: {e}"); // todo; disconnect user
                            break; // break the loop
                        },
                    }
                }

                // Tokio guarantees that `broadcast::Receiver::recv` is cancel-safe.
                resa = self.receiver.next() => {
                    match resa {
                        Some(Ok(message)) => {
                            match message {
                                Message::Text(_msg) => {}

                                Message::Ping(a) => {
                                    info!("Received a Ping from user socket, {:?}", a);
                                },

                                Message::Pong(a) => {
                                    info!("Received a Pong from user socket, {:?}", a);
                                },

                                Message::Binary(bin) => {
                                    let str = String::from_utf8(bin.to_vec()).unwrap();

                                    info!("Receive text message: {:?}", str);

                                    let ser_msg =
                                        serde_json::from_str::<MessageType>(&str).unwrap();

                                    self.hub_listener.send(ser_msg).await.unwrap(); //todo : handle -> disconnect the user
                                },

                                _ => {
                                    println!("receive message from user socket: {:?}", message);
                                }
                            }
                        }

                        Some(Err(_err)) => {
                            println!("ERROR");
                        }

                        None => {
                            println!("THENE");
                        }
                    }
                }
            }
        }
    }
}
