use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    Join { username: String },
    FindMatch,
    Move { from: String, to: String },
    Resign,
    Chat { text: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerMessage {
    #[serde(rename = "type")]
    message_type: String,
    player_id: Option<String>,
    match_id: Option<String>,
    opponent: Option<String>,
    color: Option<String>,
    reason: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Knightly Chess Client");
    println!("========================");

    // Get server address from user
    print!("Enter server address [ws://127.0.0.1:9001]: ");
    io::stdout().flush()?;
    let mut server_addr = String::new();
    io::stdin().read_line(&mut server_addr)?;
    let server_addr = server_addr.trim();
    let server_addr = if server_addr.is_empty() {
        "ws://127.0.0.1:9001".to_string()
    } else {
        server_addr.to_string()
    };

    // Connect to server
    println!("Connecting to {}...", server_addr);
    let url = Url::parse(&server_addr)?;
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to server!");

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to handle incoming messages
    let read_handle = tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.to_text().unwrap();
                        println!("\nServer: {}", text);

                        // Try to parse as structured message
                        if let Ok(parsed) = serde_json::from_str::<ServerMessage>(text) {
                            match parsed.message_type.as_str() {
                                "welcome" => {
                                    if let Some(player_id) = parsed.player_id {
                                        println!("Welcome! Your player ID: {}", player_id);
                                    }
                                }
                                "match_found" => {
                                    if let (Some(opponent), Some(color), Some(match_id)) =
                                        (parsed.opponent, parsed.color, parsed.match_id)
                                    {
                                        println!(
                                            "Match found! Opponent: {}, Color: {}, Match ID: {}",
                                            opponent, color, match_id
                                        );
                                    }
                                }
                                "error" => {
                                    if let Some(reason) = parsed.reason {
                                        println!("Error: {}", reason);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // Main loop for sending messages
    println!("\nAvailable commands:");
    println!("  join <username>    - Join the server");
    println!("  findmatch          - Find a match");
    println!("  move <from> <to>   - Make a move (e.g., move e2 e4)");
    println!("  chat <message>     - Send chat message");
    println!("  resign             - Resign from current game");
    println!("  quit               - Exit client");
    println!();

    loop {
        print!("➡️  Enter command: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "quit" | "exit" => {
                println!("👋 Goodbye!");
                break;
            }
            "join" => {
                if parts.len() >= 2 {
                    let username = parts[1..].join(" ");
                    let message = ClientMessage::Join { username };
                    send_message(&mut write, &message).await?;
                } else {
                    println!("Usage: join <username>");
                }
            }
            "findmatch" | "find" => {
                let message = ClientMessage::FindMatch;
                send_message(&mut write, &message).await?;
                println!("🔍 Searching for a match...");
            }
            "move" => {
                if parts.len() >= 3 {
                    let from = parts[1].to_string();
                    let to = parts[2].to_string();
                    let message = ClientMessage::Move { from, to };
                    send_message(&mut write, &message).await?;
                    println!("♟️  Sent move: {} -> {}", parts[1], parts[2]);
                } else {
                    println!("Usage: move <from> <to> (e.g., move e2 e4)");
                }
            }
            "chat" => {
                if parts.len() >= 2 {
                    let text = parts[1..].join(" ");
                    let message = ClientMessage::Chat { text };
                    send_message(&mut write, &message).await?;
                } else {
                    println!("Usage: chat <message>");
                }
            }
            "resign" => {
                let message = ClientMessage::Resign;
                send_message(&mut write, &message).await?;
                println!("Resigned from current game");
            }
            "help" => {
                print_help();
            }
            _ => {
                println!(
                    "Unknown command: {}. Type 'help' for available commands.",
                    command
                );
            }
        }
    }

    // Cleanup
    read_handle.abort();
    Ok(())
}

async fn send_message(
    write: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    message: &ClientMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(message)?;
    write.send(Message::Text(json)).await?;
    Ok(())
}

fn print_help() {
    println!("\n📖 Available Commands:");
    println!("  join <username>    - Register with a username");
    println!("  findmatch          - Enter matchmaking queue");
    println!("  move <from> <to>   - Make a chess move");
    println!("  chat <message>     - Send chat to opponent");
    println!("  resign             - Resign from current game");
    println!("  help               - Show this help");
    println!("  quit               - Exit the client");
    println!();
}
