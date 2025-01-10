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

    // write message to client socket connection
    pub async fn write_pump(&mut self) {
        while let Ok(msg) = self.hub_sender.recv().await {
            if msg.to_user == self.user_id.to_string() {
                let msg = serde_json::to_string(&msg).unwrap(); // todo; handle graciously
                self.sender.send(Message::Text(msg)).await.unwrap();
            }
        }
    }

    // this would be called by the hub
    // read from client, the will be sent to hub to be broadcasted
    pub async fn read_pump(&mut self) {
        while let Some(Ok(message)) = self.receiver.next().await {
            match message {
                Message::Ping(a) => {
                    info!("Received a Ping from user socket, {:?}", a);
                },

                Message::Pong(a) => {
                    info!("Received a Pong from user socket, {:?}", a);
                },

                Message::Text(text) => {
                    let ser_msg =
                        serde_json::from_str::<MessageType>(&text).unwrap();

                    info!("Receive text message: {:?}", ser_msg);

                    self.hub_listener.send(ser_msg).await.unwrap(); //todo : handle
                },

                Message::Binary(bin) => {
                    let str = String::from_utf8(bin).unwrap();

                    info!("Receive text message: {:?}", str);

                    let ser_msg =
                        serde_json::from_str::<MessageType>(&str).unwrap();

                    self.hub_listener.send(ser_msg).await.unwrap(); //todo : handle
                },

                _ => println!("Unhandled message: {:?}", message),
            }
        }
    }
}
