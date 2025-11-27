use eframe::egui;
use engine::{boardsquare::BoardSquare, chessmove::ChessMove};
use env_logger::Env;
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<(), eframe::Error> {
    // Set up logging
    let env = Env::default().filter_or("MY_LOG_LEVEL", "INFO");
    env_logger::init_from_env(env);
    warn!("Initialized logger");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(false)
            .with_min_inner_size(egui::vec2(800.0, 600.0))
            .with_inner_size(egui::vec2(1920.0, 1080.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Knightly",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "symbols".to_owned(),
                egui::FontData::from_static(include_bytes!("../fonts/DejaVuSans.ttf")).into(),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "symbols".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(ChessApp::default()))
        }),
    )
}

// Server message types (from your connection.rs)
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage2 {
    GameEnd {
        winner: String,
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

// Client event types (from your connection.rs)
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Join { username: String },
    FindMatch,
    Move { step: ChessMove },
    Resign,
    Chat { text: String },
    RequestLegalMoves { fen: String },
}

// Game state
#[derive(Debug, Clone)]
struct GameState {
    fen: String,
    player_color: Option<String>,
    opponent_name: Option<String>,
    match_id: Option<Uuid>,
    game_over: Option<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            player_color: None,
            opponent_name: None,
            match_id: None,
            game_over: None,
        }
    }
}

// UI state
enum AppState {
    MainMenu,
    Connecting,
    FindingMatch,
    InGame,
    GameOver,
}

struct ChessApp {
    state: AppState,
    game_state: Arc<Mutex<GameState>>,
    server_port: String,
    username: String,

    // Channels for communication with network tasks
    tx_to_network: Option<mpsc::UnboundedSender<ClientEvent>>,
    rx_from_network: Option<mpsc::UnboundedReceiver<ServerMessage2>>,

    // UI state
    selected_square: Option<(usize, usize)>,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            state: AppState::MainMenu,
            game_state: Arc::new(Mutex::new(GameState::default())),
            server_port: "9001".to_string(),
            username: "Player".to_string(),
            tx_to_network: None,
            rx_from_network: None,
            selected_square: None,
        }
    }
}

impl ChessApp {
    fn connect_to_server(&mut self) {
        let server_port = self.server_port.clone();
        let username = self.username.clone();
        let game_state = self.game_state.clone();

        // Create channels for communication
        let (tx_to_network, rx_from_ui) = mpsc::unbounded_channel();
        let (tx_to_ui, rx_from_network) = mpsc::unbounded_channel();

        self.tx_to_network = Some(tx_to_network);
        self.rx_from_network = Some(rx_from_network);

        self.state = AppState::Connecting;

        // Spawn network connection task
        tokio::spawn(async move {
            if let Err(e) =
                Self::network_handler(server_port, username, rx_from_ui, tx_to_ui, game_state).await
            {
                error!("Network handler error: {}", e);
            }
        });
    }

    async fn network_handler(
        server_port: String,
        username: String,
        mut rx_from_ui: mpsc::UnboundedReceiver<ClientEvent>,
        tx_to_ui: mpsc::UnboundedSender<ServerMessage2>,
        game_state: Arc<Mutex<GameState>>,
    ) -> anyhow::Result<()> {
        // Build WebSocket URL
        let server_address = format!("ws://127.0.0.1:{}", server_port);
        let url = Url::parse(&server_address)?;

        info!("Connecting to: {}", server_address);
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Send initial join message and immediately send FindMatch
        let join_event = ClientEvent::Join { username };
        write
            .send(Message::Text(serde_json::to_string(&join_event)?))
            .await?;
        info!("Sent Join event");

        // Send FindMatch immediately after joining
        let find_match_event = ClientEvent::FindMatch;
        write
            .send(Message::Text(serde_json::to_string(&find_match_event)?))
            .await?;
        info!("Sent FindMatch event");

        // Spawn reader task
        let tx_to_ui_clone = tx_to_ui.clone();
        let game_state_clone = game_state.clone();
        let reader_handle = tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) if msg.is_text() => {
                        let text = msg.to_text().unwrap();
                        info!("Received: {}", text);

                        if let Ok(server_msg) = serde_json::from_str::<ServerMessage2>(text) {
                            // Update game state
                            if let Ok(mut state) = game_state_clone.lock() {
                                match &server_msg {
                                    ServerMessage2::UIUpdate { fen } => {
                                        state.fen = fen.clone();
                                    }
                                    ServerMessage2::MatchFound {
                                        color,
                                        opponent_name,
                                        ..
                                    } => {
                                        state.player_color = Some(color.clone());
                                        state.opponent_name = Some(opponent_name.clone());
                                    }
                                    ServerMessage2::GameEnd { winner } => {
                                        state.game_over = Some(winner.clone());
                                    }
                                    _ => {}
                                }
                            }

                            // Send to UI
                            if let Err(e) = tx_to_ui_clone.send(server_msg) {
                                error!("Failed to send to UI: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Writer task (main thread)
        while let Some(event) = rx_from_ui.recv().await {
            let message = serde_json::to_string(&event)?;
            write.send(Message::Text(message)).await?;
            info!("Sent event to server: {:?}", event);
        }

        // Wait for reader to finish
        let _ = reader_handle.await;

        Ok(())
    }

    fn handle_click(&mut self, row: usize, col: usize) {
        if let Some((from_row, from_col)) = self.selected_square {
            // Send move to server
            if let Some(tx) = &self.tx_to_network {
                let chess_move = ChessMove::Quiet {
                    piece_type: engine::piecetype::PieceType::WhiteKing,
                    from_square: BoardSquare { x: 0, y: 1 },
                    to_square: BoardSquare { x: 2, y: 2 },
                    promotion_piece: None,
                };
                let move_event = ClientEvent::Move { step: chess_move };
                let _ = tx.send(move_event);
            }
            self.selected_square = None;
        } else {
            // Select square
            self.selected_square = Some((row, col));
        }
    }

    fn fen_to_board(&self, fen: &str) -> [[char; 8]; 8] {
        let mut board = [[' '; 8]; 8];
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let board_str = parts[0];

        let mut row = 0;
        let mut col = 0;

        for c in board_str.chars() {
            if c == '/' {
                row += 1;
                col = 0;
            } else if c.is_digit(10) {
                col += c.to_digit(10).unwrap() as usize;
            } else {
                if row < 8 && col < 8 {
                    board[row][col] = c;
                }
                col += 1;
            }
        }

        board
    }

    fn chess_char_to_piece(&self, c: char) -> &'static str {
        match c {
            'K' => "♔",
            'Q' => "♕",
            'R' => "♖",
            'B' => "♗",
            'N' => "♘",
            'P' => "♙",
            'k' => "♚",
            'q' => "♛",
            'r' => "♜",
            'b' => "♝",
            'n' => "♞",
            'p' => "♟︎",
            _ => "",
        }
    }

    fn process_network_messages(&mut self) {
        if let Some(rx) = &mut self.rx_from_network {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    ServerMessage2::MatchFound { .. } => {
                        info!("Match found! Transitioning to InGame state");
                        self.state = AppState::InGame;
                    }
                    ServerMessage2::GameEnd { .. } => {
                        info!("Game over! Transitioning to GameOver state");
                        self.state = AppState::GameOver;
                    }
                    ServerMessage2::Ok { response } => {
                        info!("Server OK response: {:?}", response);
                        // When we get the OK response, transition to FindingMatch state
                        // This shows the "Finding Match..." screen while we wait
                        if matches!(self.state, AppState::Connecting) {
                            self.state = AppState::FindingMatch;
                        }
                    }
                    ServerMessage2::UIUpdate { fen } => {
                        info!("Board updated with FEN: {}", fen);
                        // UI will automatically redraw with new FEN
                    }
                }
            }
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process incoming network messages
        self.process_network_messages();

        // Get current game state
        let game_state = self.game_state.lock().unwrap().clone();

        match self.state {
            AppState::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("♞ Knightly ♞");
                        ui.add_space(30.0);

                        ui.horizontal(|ui| {
                            ui.label("Username:");
                            ui.text_edit_singleline(&mut self.username);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Server Port:");
                            ui.text_edit_singleline(&mut self.server_port);
                        });

                        ui.add_space(20.0);

                        if ui.button("Connect & Play").clicked() {
                            self.connect_to_server();
                        }

                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            }

            AppState::Connecting => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Connecting to Server...");
                        ui.add_space(20.0);
                        ui.spinner();
                    });
                });
            }

            AppState::FindingMatch => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Finding Match...");
                        ui.add_space(20.0);
                        ui.label("Waiting for an opponent...");
                        ui.spinner();
                    });
                });
            }

            AppState::InGame => {
                // Draw menu bar
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Main Menu").clicked() {
                            *self = ChessApp::default();
                        }

                        if ui.button("Resign").clicked() {
                            if let Some(tx) = &self.tx_to_network {
                                let _ = tx.send(ClientEvent::Resign);
                            }
                        }

                        ui.separator();

                        if let Some(color) = &game_state.player_color {
                            ui.label(format!("You are: {}", color));
                        }

                        if let Some(opponent) = &game_state.opponent_name {
                            ui.label(format!("vs: {}", opponent));
                        }
                    });
                });

                // Draw chess board
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        let board = self.fen_to_board(&game_state.fen);
                        let is_white = game_state
                            .player_color
                            .as_ref()
                            .map_or(true, |c| c == "white");

                        let available_size = ui.available_size();
                        let board_size = available_size.x.min(available_size.y) * 0.9;
                        let tile_size = board_size / 8.0;

                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(board_size, board_size),
                            egui::Sense::click(),
                        );

                        let board_top_left = response.rect.left_top();

                        // Draw board and pieces
                        for row in 0..8 {
                            for col in 0..8 {
                                let (display_row, display_col) = if is_white {
                                    (7 - row, col)
                                } else {
                                    (row, 7 - col)
                                };

                                let color = if (row + col) % 2 == 0 {
                                    egui::Color32::from_rgb(240, 217, 181) // Light
                                } else {
                                    egui::Color32::from_rgb(181, 136, 99) // Dark
                                };

                                let rect = egui::Rect::from_min_size(
                                    egui::Pos2::new(
                                        board_top_left.x + col as f32 * tile_size,
                                        board_top_left.y + row as f32 * tile_size,
                                    ),
                                    egui::Vec2::new(tile_size, tile_size),
                                );

                                painter.rect_filled(rect, 0.0, color);

                                // Draw piece
                                let piece_char = board[display_row][display_col];
                                if piece_char != ' ' {
                                    let symbol = self.chess_char_to_piece(piece_char);
                                    let font_id = egui::FontId::proportional(tile_size * 0.8);
                                    let text_color = if piece_char.is_uppercase() {
                                        egui::Color32::WHITE
                                    } else {
                                        egui::Color32::BLACK
                                    };

                                    painter.text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        symbol,
                                        font_id,
                                        text_color,
                                    );
                                }

                                // Draw selection
                                if let Some((sel_row, sel_col)) = self.selected_square {
                                    if sel_row == display_row && sel_col == display_col {
                                        painter.rect_stroke(
                                            rect,
                                            0.0,
                                            egui::Stroke::new(3.0, egui::Color32::RED),
                                            egui::StrokeKind::Middle,
                                        );
                                    }
                                }

                                // Handle clicks
                                if response.clicked() {
                                    if let Some(click_pos) = ui.ctx().pointer_interact_pos() {
                                        if rect.contains(click_pos) {
                                            self.handle_click(display_row, display_col);
                                        }
                                    }
                                }
                            }
                        }
                    });
                });
            }

            AppState::GameOver => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Game Over");
                        ui.add_space(20.0);

                        if let Some(reason) = &game_state.game_over {
                            ui.label(format!("Result: {}", reason));
                        }

                        ui.add_space(20.0);

                        if ui.button("Back to Main Menu").clicked() {
                            *self = ChessApp::default();
                        }
                    });
                });
            }
        }

        // Request repaint to keep UI responsive
        ctx.request_repaint();
    }
}
