use crate::{
    board::Board,
    chess::{ChessMove, Piece, Square},
};

impl Square {
    pub fn is_corner(self) -> bool {
        if (self.file == 0 || self.file == 7) && (self.rank == 0 || self.rank == 7) {
            return true;
        }
        false
    }
}
impl Board {
    // codex resume 019fbc54-c741-78b2-a33e-90ab75d843f2
    pub fn get(&self, square: Square) -> Option<Piece> {
        self.board[square.rank as usize][square.file as usize]
    }
    pub fn pseudo_legal_moves(&self, square: Square) -> Option<Vec<ChessMove>> {
        if let Some(piece) = &self.get(square) {
            match piece.kind {
                crate::chess::PieceKind::Knight => {
                    let mut possible_moves = vec![];
                    if square.is_corner() {
                        // corners only two directions
                        if square.file == 0 {
                            let square_to = Square::new(square.file + 2, square.rank + 1);
                            let square_to_another = Square::new(square.file + 1, square.rank + 2);
                            possible_moves.extend(vec![
                                ChessMove::new(square, square_to),
                                ChessMove::new(square, square_to_another),
                            ]);
                        } else {
                            let square_to = Square::new(square.file - 2, square.rank - 1);
                            let square_to_another = Square::new(square.file - 1, square.rank - 2);
                            possible_moves.extend(vec![
                                ChessMove::new(square, square_to),
                                ChessMove::new(square, square_to_another),
                            ]);
                        }
                    } else {
                        // middle of the board can go in 4 direction
                    }
                    let square_top_left = Square::new(square.file + 2, square.rank + 1);
                    let square_top_right = Square::new(square.file + 2, square.rank - 1);
                    let square_bottom_left = Square::new(square.file - 2, square.rank - 1);
                    let square_bottom_right = Square::new(square.file - 2, square.rank - 1);
                    let square_left_top = Square::new(square.file - 1, square.rank + 2);
                    let square_left_bottom = Square::new(square.file - 1, square.rank - 2);
                    let square_right_top = Square::new(square.file + 1, square.rank + 2);
                    let square_right_bottom = Square::new(square.file + 1, square.rank - 2);
                    // TODO src/chess.rs:49 compares only two of the four move coordinates, so different moves may compare equal.
                    possible_moves.extend(vec![
                        ChessMove::new(square, square_top_left),
                        ChessMove::new(square, square_top_right),
                        ChessMove::new(square, square_bottom_left),
                        ChessMove::new(square, square_bottom_right),
                        ChessMove::new(square, square_left_top),
                        ChessMove::new(square, square_left_bottom),
                        ChessMove::new(square, square_right_top),
                        ChessMove::new(square, square_right_bottom),
                    ]);
                    Some(possible_moves)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn psuedo_legal_moves_accepts_middle_knight_moves() {
        todo!()
    }
    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves() {
        todo!()
    }
    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves_with_opponents() {
        todo!()
    }
}
