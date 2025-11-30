use eframe::egui;
use engine::{boardsquare::BoardSquare, chessmove::ChessMove};
use env_logger::Env;
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
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
            .with_min_inner_size(egui::vec2(800.0, 800.0))
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage2 {
    GameEnd {
        winner: String,
    },
    UIUpdate {
        fen: String,
        turn_player: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Join {
        username: String,
    },
    FindMatch,
    Move {
        step: ChessMove,
        turn_player: String,
    },
    Resign,
    Chat {
        text: String,
    },
    RequestLegalMoves {
        fen: String,
    },
    CloseConnection,
}

// Game state
#[derive(Debug, Clone)]
struct GameState {
    fen: String,
    player_color: Option<String>,
    opponent_name: Option<String>,
    match_id: Option<Uuid>,
    game_over: Option<String>,
    available_moves: Option<Vec<ChessMove>>,
    turn_player: Option<String>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
            player_color: None,
            opponent_name: None,
            match_id: None,
            game_over: None,
            available_moves: None,
            turn_player: Some("white".to_string()),
        }
    }
}

// UI state
enum AppState {
    MainMenu,
    PrivatePlayConnect,
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
    server_ip: String,
    start_local_server_instance: bool,
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
            server_ip: "127.0.0.1".to_string(),
            // TODO: for the online server (reverse proxy?)
            start_local_server_instance: false,
        }
    }
}

impl ChessApp {
    fn connect_to_server(&mut self) {
        let server_port = self.server_port.clone();
        let username = self.username.clone();
        let game_state = self.game_state.clone();
        let server_address = self.server_ip.clone();

        // Create channels for communication
        let (tx_to_network, rx_from_ui) = mpsc::unbounded_channel();
        let (tx_to_ui, rx_from_network) = mpsc::unbounded_channel();

        self.tx_to_network = Some(tx_to_network);
        self.rx_from_network = Some(rx_from_network);

        self.state = AppState::Connecting;

        // Spawn network connection task
        tokio::spawn(async move {
            if let Err(e) = Self::network_handler(
                server_port,
                server_address,
                username,
                rx_from_ui,
                tx_to_ui,
                game_state,
            )
            .await
            {
                error!("Network handler error: {}", e);
            }
        });
    }

    async fn network_handler(
        server_port: String,
        server_ip: String,
        username: String,
        mut rx_from_ui: mpsc::UnboundedReceiver<ClientEvent>,
        tx_to_ui: mpsc::UnboundedSender<ServerMessage2>,
        game_state: Arc<Mutex<GameState>>,
    ) -> anyhow::Result<()> {
        // Build WebSocket URL
        let server_address = format!("ws://{}:{}", server_ip, server_port);
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
                                    ServerMessage2::UIUpdate { fen, turn_player } => {
                                        state.fen = fen.clone();
                                        state.turn_player = Some(turn_player.clone());
                                    }
                                    ServerMessage2::MatchFound {
                                        color,
                                        opponent_name,
                                        match_id,
                                    } => {
                                        state.player_color = Some(color.clone());
                                        state.opponent_name = Some(opponent_name.clone());
                                        state.match_id = Some(match_id.clone());
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
                let player_color = self.game_state.lock().unwrap().player_color.clone();

                //check if its the players turn
                if self.game_state.lock().unwrap().turn_player != player_color {
                    warn!("it is not the current players turn!");
                    self.selected_square = None;
                    return;
                }

                // TODO: kinyerni a tenyleges kivalasztott babut
                let chess_move = ChessMove::Quiet {
                    piece_type: engine::piecetype::PieceType::WhiteKing,
                    from_square: BoardSquare { x: 0, y: 1 },
                    to_square: BoardSquare { x: 2, y: 2 },
                    promotion_piece: None,
                };

                let move_event = ClientEvent::Move {
                    step: chess_move,
                    turn_player: if player_color == Some("white".to_string()) {
                        "black".to_string()
                    } else {
                        "white".to_string()
                    },
                };

                let _ = tx.send(move_event);
            }

            self.selected_square = None;
        } else {
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
            'K' => "♚",
            'Q' => "♛",
            'R' => "♜",
            'B' => "♝",
            'N' => "♞",
            'P' => "♟︎",
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
                    ServerMessage2::UIUpdate { fen, turn_player } => {
                        info!("Board updated with FEN: {}", fen);
                        // UI will automatically redraw with new FEN
                        if let Ok(mut game_state) = self.game_state.lock() {
                            game_state.fen = fen;
                            game_state.turn_player = Some(turn_player);
                        }
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

                        ui.add_space(20.0);

                        if ui.button("Online Play").clicked() {
                            // TODO: change to dns
                            self.server_ip = "127.0.0.1".to_string();
                            self.connect_to_server();
                        }

                        if ui.button("Private Play").clicked() {
                            self.state = AppState::PrivatePlayConnect;
                        }

                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            }

            AppState::PrivatePlayConnect => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Server ip address");
                            ui.text_edit_singleline(&mut self.server_ip);
                        });

                        ui.horizontal(|ui| {
                            ui.label("Server Port:");
                            ui.text_edit_singleline(&mut self.server_port);
                        });

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.start_local_server_instance, "Host Server");
                        });

                        ui.add_space(20.0);

                        if ui.button("Play").clicked() {
                            if self.start_local_server_instance == true {
                                let path = if cfg!(windows) {
                                    "./server.exe"
                                } else {
                                    "./server"
                                };

                                if !Path::new(path).exists() {
                                    error!("Server binary does not exist, cfg: {}", path);
                                } else {
                                    let _ = Command::new(path).spawn();
                                    std::thread::sleep(std::time::Duration::from_secs(1));
                                }
                            }
                            self.connect_to_server();
                        }
                    })
                });
            }

            AppState::Connecting => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Connecting to Server...");
                        ui.add_space(20.0);
                        ui.spinner();
                    });

                    if ui.button("Cancel").clicked() {
                        info!("Returning to menu from before connecting to the server");
                        self.state = AppState::MainMenu;
                    }
                });
            }

            AppState::FindingMatch => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Finding Match...");
                        ui.add_space(20.0);
                        ui.label("Waiting for an opponent...");
                        ui.spinner();

                        ui.add_space(20.0);
                        if ui.button("cancel").clicked() {
                            if let Some(tx) = &self.tx_to_network {
                                warn!("Closing connection to server, cancelled match findig!");
                                tx.send(ClientEvent::CloseConnection);
                                self.state = AppState::MainMenu;
                            }
                        }
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
                                    (row, col)
                                } else {
                                    (7 - row, 7 - col)
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_game_state() {
        let game_state = GameState::default();
        assert_eq!(
            game_state.fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
        assert_eq!(game_state.player_color, None);
        assert_eq!(game_state.opponent_name, None);
        assert_eq!(game_state.match_id, None);
        assert_eq!(game_state.game_over, None);
    }

    #[test]
    fn test_fen_to_board_starting_position() {
        let app = ChessApp::default();
        let board = app.fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        // Test black pieces on rank 0
        assert_eq!(board[0][0], 'r');
        assert_eq!(board[0][1], 'n');
        assert_eq!(board[0][2], 'b');
        assert_eq!(board[0][3], 'q');
        assert_eq!(board[0][4], 'k');
        assert_eq!(board[0][5], 'b');
        assert_eq!(board[0][6], 'n');
        assert_eq!(board[0][7], 'r');

        // Test black pawns on rank 1
        for col in 0..8 {
            assert_eq!(board[1][col], 'p');
        }

        // Test empty squares in the middle
        for row in 2..6 {
            for col in 0..8 {
                assert_eq!(board[row][col], ' ');
            }
        }

        // Test white pawns on rank 6
        for col in 0..8 {
            assert_eq!(board[6][col], 'P');
        }

        // Test white pieces on rank 7
        assert_eq!(board[7][0], 'R');
        assert_eq!(board[7][1], 'N');
        assert_eq!(board[7][2], 'B');
        assert_eq!(board[7][3], 'Q');
        assert_eq!(board[7][4], 'K');
        assert_eq!(board[7][5], 'B');
        assert_eq!(board[7][6], 'N');
        assert_eq!(board[7][7], 'R');
    }

    #[test]
    fn test_fen_to_board_with_numbers() {
        let app = ChessApp::default();
        let board = app.fen_to_board("4k3/8/8/8/8/8/8/4K3");

        // Test empty squares around kings
        for row in 0..8 {
            for col in 0..8 {
                if (row == 0 && col == 4) || (row == 7 && col == 4) {
                    continue; // Skip king positions
                }
                assert_eq!(board[row][col], ' ');
            }
        }

        // Test king positions
        assert_eq!(board[0][4], 'k'); // black king
        assert_eq!(board[7][4], 'K'); // white king
    }

    #[test]
    fn test_chess_char_to_piece() {
        let app = ChessApp::default();

        // Test white pieces
        assert_eq!(app.chess_char_to_piece('K'), "♚");
        assert_eq!(app.chess_char_to_piece('Q'), "♛");
        assert_eq!(app.chess_char_to_piece('R'), "♜");
        assert_eq!(app.chess_char_to_piece('B'), "♝");
        assert_eq!(app.chess_char_to_piece('N'), "♞");
        assert_eq!(app.chess_char_to_piece('P'), "♟︎");

        // Test black pieces
        assert_eq!(app.chess_char_to_piece('k'), "♚");
        assert_eq!(app.chess_char_to_piece('q'), "♛");
        assert_eq!(app.chess_char_to_piece('r'), "♜");
        assert_eq!(app.chess_char_to_piece('b'), "♝");
        assert_eq!(app.chess_char_to_piece('n'), "♞");
        assert_eq!(app.chess_char_to_piece('p'), "♟︎");

        // Test invalid piece
        assert_eq!(app.chess_char_to_piece('X'), "");
        assert_eq!(app.chess_char_to_piece(' '), "");
    }

    #[test]
    fn test_chess_app_default() {
        let app = ChessApp::default();

        assert_eq!(app.server_port, "9001");
        assert_eq!(app.username, "Player");
        assert!(app.tx_to_network.is_none());
        assert!(app.rx_from_network.is_none());
        assert!(app.selected_square.is_none());

        // Verify initial state is MainMenu
        match app.state {
            AppState::MainMenu => (),
            _ => panic!("Expected initial state to be MainMenu"),
        }
    }

    #[test]
    fn test_game_state_clone() {
        let mut original = GameState::default();
        original.player_color = Some("white".to_string());
        original.opponent_name = Some("Opponent".to_string());
        original.match_id = Some(Uuid::new_v4());
        original.game_over = Some("Checkmate".to_string());

        let cloned = original.clone();

        assert_eq!(original.fen, cloned.fen);
        assert_eq!(original.player_color, cloned.player_color);
        assert_eq!(original.opponent_name, cloned.opponent_name);
        assert_eq!(original.match_id, cloned.match_id);
        assert_eq!(original.game_over, cloned.game_over);
    }

    #[test]
    fn test_handle_click_selection() {
        let mut app = ChessApp::default();

        // Initially no square should be selected
        assert_eq!(app.selected_square, None);

        // Click on a square should select it
        app.handle_click(3, 4);
        assert_eq!(app.selected_square, Some((3, 4)));

        // Click on another square should deselect and send move (if tx exists)
        // Since we don't have a real tx in tests, we just verify the selection is cleared
        app.handle_click(4, 4);
        assert_eq!(app.selected_square, None);
    }

    #[test]
    fn test_process_network_messages_match_found() {
        let mut app = ChessApp::default();
        let (tx, mut rx) = mpsc::unbounded_channel();
        app.rx_from_network = Some(rx);

        // Send a MatchFound message
        let match_id = Uuid::new_v4();
        let message = ServerMessage2::MatchFound {
            match_id,
            color: "white".to_string(),
            opponent_name: "TestOpponent".to_string(),
        };

        tx.send(message).unwrap();

        // Process the message
        app.process_network_messages();

        // State should transition to InGame
        match app.state {
            AppState::InGame => (),
            _ => panic!("Expected state to transition to InGame"),
        }
    }

    #[test]
    fn test_process_network_messages_game_over() {
        let mut app = ChessApp::default();
        let (tx, mut rx) = mpsc::unbounded_channel();
        app.rx_from_network = Some(rx);

        // Send a GameEnd message
        let message = ServerMessage2::GameEnd {
            winner: "White won by checkmate".to_string(),
        };

        tx.send(message).unwrap();

        // Process the message
        app.process_network_messages();

        // State should transition to GameOver
        match app.state {
            AppState::GameOver => (),
            _ => panic!("Expected state to transition to GameOver"),
        }
    }

    #[test]
    fn test_process_network_messages_ui_update() {
        let mut app = ChessApp::default();
        let (tx, mut rx) = mpsc::unbounded_channel();
        app.rx_from_network = Some(rx);

        // Send a UIUpdate message
        let new_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
        let message = ServerMessage2::UIUpdate {
            fen: new_fen.clone(),
        };

        tx.send(message).unwrap();

        // Process the message
        app.process_network_messages();

        // Game state should be updated with new FEN
        let game_state = app.game_state.lock().unwrap();
        assert_eq!(game_state.fen, new_fen);
    }

    #[test]
    fn test_process_network_messages_ok_response() {
        let mut app = ChessApp::default();
        app.state = AppState::Connecting;
        let (tx, mut rx) = mpsc::unbounded_channel();
        app.rx_from_network = Some(rx);

        // Send an Ok message
        let message = ServerMessage2::Ok { response: Ok(()) };

        tx.send(message).unwrap();

        // Process the message
        app.process_network_messages();

        // State should transition to FindingMatch when in Connecting state
        match app.state {
            AppState::FindingMatch => (),
            _ => panic!("Expected state to transition to FindingMatch"),
        }
    }

    #[test]
    fn test_fen_edge_cases() {
        let app = ChessApp::default();

        // Test empty board
        let empty_board = app.fen_to_board("8/8/8/8/8/8/8/8");
        for row in 0..8 {
            for col in 0..8 {
                assert_eq!(empty_board[row][col], ' ');
            }
        }

        // Test FEN with multiple digit numbers
        let board = app.fen_to_board("k7/8/8/8/8/8/8/7K");
        assert_eq!(board[0][0], 'k');
        assert_eq!(board[7][7], 'K');

        // Test FEN with mixed pieces and numbers
        let board = app.fen_to_board("r3k2r/8/8/8/8/8/8/R3K2R");
        assert_eq!(board[0][0], 'r');
        assert_eq!(board[0][4], 'k');
        assert_eq!(board[0][7], 'r');
        assert_eq!(board[7][0], 'R');
        assert_eq!(board[7][4], 'K');
        assert_eq!(board[7][7], 'R');
    }

    #[tokio::test]
    async fn test_client_event_serialization() {
        // Test Join event
        let join_event = ClientEvent::Join {
            username: "test".to_string(),
        };
        let serialized = serde_json::to_string(&join_event).unwrap();
        assert!(serialized.contains("Join"));
        assert!(serialized.contains("test"));

        // Test FindMatch event
        let find_match_event = ClientEvent::FindMatch;
        let serialized = serde_json::to_string(&find_match_event).unwrap();
        assert!(serialized.contains("FindMatch"));

        // Test Move event
        let chess_move = ChessMove::Quiet {
            piece_type: engine::piecetype::PieceType::WhiteKing,
            from_square: BoardSquare { x: 0, y: 1 },
            to_square: BoardSquare { x: 2, y: 2 },
            promotion_piece: None,
        };
        let move_event = ClientEvent::Move { step: chess_move };
        let serialized = serde_json::to_string(&move_event).unwrap();
        assert!(serialized.contains("Move"));

        // Test Resign event
        let resign_event = ClientEvent::Resign;
        let serialized = serde_json::to_string(&resign_event).unwrap();
        assert!(serialized.contains("Resign"));
    }

    #[test]
    fn test_server_message_deserialization() {
        // Test Ok message
        let ok_json = r#"{"Ok":{"response":{"Ok":null}}}"#;
        let message: ServerMessage2 = serde_json::from_str(ok_json).unwrap();
        match message {
            ServerMessage2::Ok { response } => {
                assert!(response.is_ok());
            }
            _ => panic!("Expected Ok message"),
        }

        // Test MatchFound message
        let match_found_json = r#"{"MatchFound":{"match_id":"12345678-1234-1234-1234-123456789012","color":"white","opponent_name":"Test"}}"#;
        let message: ServerMessage2 = serde_json::from_str(match_found_json).unwrap();
        match message {
            ServerMessage2::MatchFound {
                match_id,
                color,
                opponent_name,
            } => {
                assert_eq!(color, "white");
                assert_eq!(opponent_name, "Test");
            }
            _ => panic!("Expected MatchFound message"),
        }

        // Test UIUpdate message
        let ui_update_json =
            r#"{"UIUpdate":{"fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"}}"#;
        let message: ServerMessage2 = serde_json::from_str(ui_update_json).unwrap();
        match message {
            ServerMessage2::UIUpdate { fen } => {
                assert!(fen.contains("rnbqkbnr"));
            }
            _ => panic!("Expected UIUpdate message"),
        }
    }
}
