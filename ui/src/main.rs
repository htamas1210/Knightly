use eframe::egui;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size(egui::vec2(400.0, 400.0)) // Minimum width, height
            .with_inner_size(egui::vec2(800.0, 600.0)), // Initial size
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

struct ChessApp {
    board: [[Piece; 8]; 8],
    selected: Option<(usize, usize)>,
    turn: Turn,
}

impl Default for ChessApp {
    fn default() -> Self {
        Self {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Resign").clicked() {
                    *self = ChessApp::default();
                }
                
                ui.separator();
                ui.label(format!("Turn: {:?}", self.turn));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.vertical_centered(|ui| {
                    let available = ui.available_size();
                    let tile = (available.x.min(available.y)) / 8.0;
                    let board_size = tile * 8.0;
                    ui.set_width(board_size);
                    ui.set_min_height(board_size);
                    let piece_label = |symbol: &str, tile: f32| {
                        let mut job = egui::text::LayoutJob::default();
                        job.append(
                    symbol,
            0.0,
                egui::TextFormat {
                        font_id: egui::FontId::proportional(tile * 0.75),
                        color: egui::Color32::BLACK,
                        ..Default::default()
        },
    );
    job
};

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

                                    let label = piece_label(piece.symbol(), tile);
                                    let mut button = egui::Button::new(label)
                                        .min_size(egui::vec2(tile, tile))
                                        .fill(color);

                                    if is_selected {
                                        button = button.stroke(
                                            egui::Stroke::new(2.0, egui::Color32::BLACK),
                                        );
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_board_setup() {
        let app = ChessApp::default();
        
        // Test that pieces are in correct starting positions
        assert!(matches!(app.board[0][0], Piece::Rook('b')));
        assert!(matches!(app.board[7][0], Piece::Rook('w')));

        assert!(matches!(app.board[1][0], Piece::Pawn('b')));
        assert!(matches!(app.board[6][0], Piece::Pawn('w')));


    }
    #[test]
    fn test_piece_symbols() {
        // Test that all piece symbols return valid strings
        assert_eq!(Piece::King('w').symbol(), "♔");
        assert_eq!(Piece::King('b').symbol(), "♚");
        assert_eq!(Piece::Empty.symbol(), "");
    }
    #[test]
    fn test_piece_selection() {
    let mut app = ChessApp::default();
    
    // Test selecting a piece, and then unselecting it
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
