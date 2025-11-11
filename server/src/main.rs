use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, connect_async};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    username: String,
    text: String,
}

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:9001"; //accept connection from anywhere
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server running on ws://{}", address);

    while let Ok((stream, _)) = listener.accept().await {
        println!("New connection!");

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut write, mut read) = ws_stream.split(); //seperate the read and write channel

            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let txt = msg.to_text().unwrap();
                    if let Ok(json) = serde_json::from_str::<Message>(txt) {
                        println!("Recieved: {:?}", json);

                        //for testing the write channel, we send back the
                        //same data the client sent
                        let reply = serde_json::to_string(&json).unwrap();
                        write
                            .send(tokio_tungstenite::tungstenite::Message::Text(reply))
                            .await
                            .unwrap();
                    }
                }
            }
        });
    }
}
