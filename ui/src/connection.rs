use engine::{chessmove::ChessMove, gameend::GameEnd};
use futures_util::StreamExt;
use local_ip_address::local_ip;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr},
};
use tokio_tungstenite::connect_async;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ServerMessage2 {
    GameEnd {
        winner: GameEnd,
    },
    UIUpdate {
        fen: String,
    },
    MatchFound {
        match_id: Uuid,
        color: String,
        opponent_name: String,
    },
    Ok {
        response: Result<(), String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    Join { username: String },
    FindMatch,
    Move { step: ChessMove, fen: String },
    Resign,
    Chat { text: String },
    RequestLegalMoves { fen: String },
}

fn get_ip_address() -> IpAddr {
    let ip = local_ip().unwrap_or(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));

    ip
}

pub async fn handle_connection(server_port: &str) -> anyhow::Result<()> {
    let address = get_ip_address();

    //start main loop
    let server_address = String::from("ws://") + &address.to_string() + ":" + server_port;
    warn!(
        "Machine IpAddress is bound for listener. Ip: {}",
        server_address
    );

    let url = Url::parse(&server_address)?;

    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    let read_handle = while let Some(message) = read.next().await {
        info!("connection");
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_text().unwrap();
                    info!("text: {}", text);

                    if let Ok(parsed) = serde_json::from_str::<ServerMessage2>(text) {
                        match parsed {
                            ServerMessage2::GameEnd { winner } => {}
                            ServerMessage2::UIUpdate { fen } => {}
                            ServerMessage2::MatchFound {
                                match_id,
                                color,
                                opponent_name,
                            } => {}
                            ServerMessage2::Ok { response } => {}
                            _ => {
                                error!("Received unkown servermessage2");
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("Error receiving message: {}", e);
            }
        }
    };

    Ok(())
}
