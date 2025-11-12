use eframe::egui;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_min_inner_size(egui::vec2(800.0, 600.0)) // Minimum width, height
            .with_inner_size(egui::vec2(1920.0, 1080.0)), // Initial size
        ..Default::default()
    };
    eframe::run_native(
    "Knightly",
    options,
    Box::new(|cc| {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "symbols".to_owned(),
            egui::FontData::from_static(include_bytes!("../../fonts/DejaVuSans.ttf")).into(),
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
}

struct ChessApp {
    fullscreen: bool,
    resolutions: Vec<(u32, u32)>,
    selected_resolution: usize,
    state:AppState,
    board: [[Piece; 8]; 8],
    selected: Option<(usize, usize)>,
    turn: Turn,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
            fullscreen:true,
            resolutions: vec![
                (1280, 720),
                (1600, 900),
                (1920, 1080),
            ],
            selected_resolution:0,
            state:AppState::MainMenu,
            board: Self::starting_board(),
            selected: None,
            turn: Turn::White,
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
}

    impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.state {
            AppState::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("♞ Knightly ♞");
                        ui.add_space(30.0);

                        if ui.add_sized([200.0, 40.0], egui::Button::new("Play")).clicked() {
                            self.state = AppState::InGame;
                        }
                        ui.add_space(8.0);
                        if ui
                            .add_sized([200.0, 40.0], egui::Button::new("Quit"))
                            .clicked()
                        {
                            std::process::exit(0);
                        }
                    });
                });
            }

            AppState::InGame => {
                // top menu bar
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        if ui.button("Main Menu").clicked() {
                            self.state = AppState::MainMenu;
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
                        let board_rect =
                            egui::Rect::from_center_size(full_avail.center(), egui::vec2(board_size, board_size));
                        ui.allocate_ui_at_rect(board_rect, |ui| {
                            let tile = ui.available_size().x.min(ui.available_size().y) / 8.0;
                            egui::Grid::new("chess_grid")
                                .spacing([0.0, 0.0])
                                .show(ui, |ui| {
                                    for row in 0..8 {
                                        for col in 0..8 {
                                            let piece = self.board[row][col];
                                            let is_selected = self.selected == Some((row, col));

                                            let color = if (row + col) % 2 == 0 {
                                                egui::Color32::from_rgb(100, 97, 97)
                                            } else {
                                                egui::Color32::from_rgb(217, 217, 217)
                                            };
                                            let rich = egui::RichText::new(piece.symbol())
                                                .font(egui::FontId::proportional(tile * 0.75));

                                            let mut button = egui::Button::new(rich)
                                                .min_size(egui::vec2(tile, tile))
                                                .fill(color);

                                            if is_selected {
                                                button = button.stroke(egui::Stroke::new(
                                                    2.0,
                                                    egui::Color32::RED,
                                                ));
                                            }

                                            if ui.add(button).clicked() {
                                                self.handle_click(row, col);
                                            }
                                        }
                                        ui.end_row();
                                    }
                                });
                        });
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
}