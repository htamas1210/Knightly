use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(false)
            .with_min_inner_size(egui::vec2(800.0, 600.0)) // Minimum width, height
            .with_inner_size(egui::vec2(7680.0, 4320.0)), // Initial size
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

#[derive(Clone, Copy, PartialEq, Debug)]
enum Piece {
    King(char),
    Queen(char),
    Rook(char),
    Bishop(char),
    Knight(char),
    Pawn(char),
    Empty,
}

impl Piece {
    fn symbol(&self) -> &'static str {
        match self {
            Piece::King('w') => "♔",
            Piece::Queen('w') => "♕",
            Piece::Rook('w') => "♖",
            Piece::Bishop('w') => "♗",
            Piece::Knight('w') => "♘",
            Piece::Pawn('w') => "♙",
            Piece::King('b') => "♚",
            Piece::Queen('b') => "♛",
            Piece::Rook('b') => "♜",
            Piece::Bishop('b') => "♝",
            Piece::Knight('b') => "♞",
            Piece::Pawn('b') => "♟︎",
            Piece::Empty => "",
            _ => "",
        }
    }
}

#[derive(PartialEq, Debug)]
enum Turn {
    White,
    Black,
}

enum AppState {
    MainMenu,
    InGame,
    Settings,
}

struct ChessApp {
    fullscreen: bool,
    resolutions: Vec<(u32, u32)>,
    selected_resolution: usize,
    state: AppState,
    board: [[Piece; 8]; 8],
    selected: Option<(usize, usize)>,
    turn: Turn,
    pending_settings: PendingSettings,
    server_port: String,
}

#[derive(Default)]
struct PendingSettings {
    fullscreen: bool,
    selected_resolution: usize,
    server_port: String,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            fullscreen: true,
            resolutions: vec![
                (1280, 720),
                (1600, 900),
                (1920, 1080),
                (2560, 1440),
                (3840, 2160),
            ],
            selected_resolution: 2, // Default to 1920x1080
            state: AppState::MainMenu,
            board: Self::starting_board(),
            selected: None,
            turn: Turn::White,
            pending_settings: PendingSettings::default(),
            server_port: "8080".to_string(), // Default port
        }
    }
}

impl ChessApp {
    fn starting_board() -> [[Piece; 8]; 8] {
        use Piece::*;
        [
            [
                Rook('b'),
                Knight('b'),
                Bishop('b'),
                Queen('b'),
                King('b'),
                Bishop('b'),
                Knight('b'),
                Rook('b'),
            ],
            [Pawn('b'); 8],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Pawn('w'); 8],
            [
                Rook('w'),
                Knight('w'),
                Bishop('w'),
                Queen('w'),
                King('w'),
                Bishop('w'),
                Knight('w'),
                Rook('w'),
            ],
        ]
    }

    fn handle_click(&mut self, row: usize, col: usize) {
        if let Some((r, c)) = self.selected {
            let piece = self.board[r][c];
            self.board[r][c] = Piece::Empty;
            self.board[row][col] = piece;
            self.selected = None;
            self.turn = if self.turn == Turn::White {
                Turn::Black
            } else {
                Turn::White
            };
        } else {
            if self.board[row][col] != Piece::Empty {
                self.selected = Some((row, col));
            }
        }
    }

    fn apply_settings(&mut self, ctx: &egui::Context) {
        self.fullscreen = self.pending_settings.fullscreen;
        self.selected_resolution = self.pending_settings.selected_resolution;
        self.server_port = self.pending_settings.server_port.clone();
        
        if let Some(resolution) = self.resolutions.get(self.selected_resolution) {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
                egui::Vec2::new(resolution.0 as f32, resolution.1 as f32)
            ));
        }
        
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    fn enter_settings(&mut self) {
        self.pending_settings.fullscreen = self.fullscreen;
        self.pending_settings.selected_resolution = self.selected_resolution;
        self.pending_settings.server_port = self.server_port.clone();
        self.state = AppState::Settings;
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.state {
            AppState::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("♞ Knightly ♞");
                        ui.add_space(30.0);

                        if ui.add_sized([300.0, 60.0], egui::Button::new("Play")).clicked() {
                            self.state = AppState::InGame;
                        }
                        ui.add_space(8.0);
                        
                        if ui.add_sized([300.0, 60.0], egui::Button::new("Settings")).clicked() {
                            self.enter_settings();
                        }
                        ui.add_space(8.0);
                        
                        if ui
                            .add_sized([300.0, 60.0], egui::Button::new("Quit"))
                            .clicked()
                        {
                            std::process::exit(0);
                        }
                    });
                });
            }

            AppState::Settings => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Settings");
                        ui.add_space(30.0);

                        // Fullscreen toggle
                        ui.horizontal(|ui| {
                            ui.label("Fullscreen:");
                            if ui.checkbox(&mut self.pending_settings.fullscreen, "").changed() {
                                // If enabling fullscreen, we might want to disable resolution selection
                            }
                        });
                        ui.add_space(10.0);

                        // Resolution dropdown
                        ui.horizontal(|ui| {
                            ui.label("Resolution:");
                            egui::ComboBox::new("resolution_combo", "")
                                .selected_text(format!(
                                    "{}x{}",
                                    self.resolutions[self.pending_settings.selected_resolution].0,
                                    self.resolutions[self.pending_settings.selected_resolution].1
                                ))
                                .show_ui(ui, |ui| {
                                    for (i, &(width, height)) in self.resolutions.iter().enumerate() {
                                        ui.selectable_value(
                                            &mut self.pending_settings.selected_resolution,
                                            i,
                                            format!("{}x{}", width, height),
                                        );
                                    }
                                });
                        });
                        ui.add_space(10.0);

                        // Server port input field
                        ui.horizontal(|ui| {
                            ui.label("Local Server Port:");
                            ui.add(egui::TextEdit::singleline(&mut self.pending_settings.server_port)
                                .desired_width(100.0)
                                .hint_text("8080"));
                        });
                        ui.add_space(30.0);

                        // Apply and Cancel buttons
                        ui.horizontal(|ui| {
                            if ui.add_sized([140.0, 40.0], egui::Button::new("Apply")).clicked() {
                                self.apply_settings(ctx);
                                self.state = AppState::MainMenu;
                            }
                            
                            if ui.add_sized([140.0, 40.0], egui::Button::new("Cancel")).clicked() {
                                self.state = AppState::MainMenu;
                            }
                        });
                    });
                });
            }

            AppState::InGame => {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Main Menu").clicked() {
                            self.state = AppState::MainMenu;
                        }
                        if ui.button("Settings").clicked() {
                            self.enter_settings();
                        }
                        if ui.button("New Game").clicked() {
                            *self = ChessApp::default();
                            self.state = AppState::InGame;
                        }
                        ui.separator();
                        ui.label(format!("Turn: {:?}", self.turn));
                    });
                });
                
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        let full_avail = ui.available_rect_before_wrap();
                        let board_tile = (full_avail.width().min(full_avail.height())) / 8.0;
                        let board_size = board_tile * 8.0;
                        
                        // Create a child UI at the board position
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(board_size, board_size),
                            egui::Sense::click()
                        );
                        
                        let board_rect = egui::Rect::from_center_size(
                            full_avail.center(),
                            egui::vec2(board_size, board_size)
                        );
                        
                        // Draw the chess board
                        let tile_size = board_size / 8.0;
                        for row in 0..8 {
                            for col in 0..8 {
                                let color = if (row + col) % 2 == 0 {
                                    egui::Color32::from_rgb(100, 97, 97)
                                } else {
                                    egui::Color32::from_rgb(217, 217, 217)
                                };
                                
                                let rect = egui::Rect::from_min_size(
                                    egui::Pos2::new(
                                        board_rect.min.x + col as f32 * tile_size,
                                        board_rect.min.y + row as f32 * tile_size
                                    ),
                                    egui::Vec2::new(tile_size, tile_size)
                                );
                                
                                painter.rect_filled(rect, 0.0, color);
                                
                                // Draw piece
                                let piece = self.board[row][col];
                                if piece != Piece::Empty {
                                    let symbol = piece.symbol();
                                    let font_id = egui::FontId::proportional(tile_size * 0.75);
                                    painter.text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        symbol,
                                        font_id,
                                        if matches!(piece, Piece::King('w') | Piece::Queen('w') | Piece::Rook('w') | Piece::Bishop('w') | Piece::Knight('w') | Piece::Pawn('w')) {
                                            egui::Color32::WHITE
                                        } else {
                                            egui::Color32::BLACK
                                        }
                                    );
                                }
                                
                                // Draw selection highlight
                                if self.selected == Some((row, col)) {
                                    painter.rect_stroke(
                                        rect, 
                                        0.0, 
                                        egui::Stroke::new(3.0, egui::Color32::RED),
                                        egui::StrokeKind::Inside
                                    );
                                }
                                
                                // Handle clicks
                                if ui.ctx().input(|i| i.pointer.primary_clicked()) {
                                    let click_pos = ui.ctx().input(|i| i.pointer.interact_pos()).unwrap();
                                    if rect.contains(click_pos) {
                                        self.handle_click(row, col);
                                    }
                                }
                            }
                        }
                    });
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_board_setup() {
        let app = ChessApp::default();
        assert!(matches!(app.board[0][0], Piece::Rook('b')));
        assert!(matches!(app.board[7][0], Piece::Rook('w')));

        assert!(matches!(app.board[1][0], Piece::Pawn('b')));
        assert!(matches!(app.board[6][0], Piece::Pawn('w')));
    }
    
    #[test]
    fn test_piece_symbols() {
        assert_eq!(Piece::King('w').symbol(), "♔");
        assert_eq!(Piece::King('b').symbol(), "♚");
        assert_eq!(Piece::Empty.symbol(), "");
    }
    
    #[test]
    fn test_piece_selection() {
        let mut app = ChessApp::default();
        app.handle_click(6, 0);
        assert_eq!(app.selected, Some((6, 0)));
        app.handle_click(6, 0);
        assert_eq!(app.selected, None);
    }
    
    #[test]
    fn test_piece_movement() {
        let mut app = ChessApp::default();
        // Select and move a piece
        app.handle_click(6, 0); // Select white pawn
        app.handle_click(5, 0); // Move to empty square
        assert_eq!(app.board[6][0], Piece::Empty);
        assert!(matches!(app.board[5][0], Piece::Pawn('w')));
    }

    #[test]
    fn test_turn_switching() {
        let mut app = ChessApp::default();
        assert_eq!(app.turn, Turn::White);
        app.handle_click(6, 0); // White selects
        app.handle_click(5, 0); // White moves
        assert_eq!(app.turn, Turn::Black); // Should now be Black's turn
    }
    
    #[test]
    fn test_server_port_default() {
        let app = ChessApp::default();
        assert_eq!(app.server_port, "8080");
    }
}
