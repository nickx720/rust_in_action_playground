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
        self.board[square.file as usize][square.rank as usize]
    }
    pub fn pseudo_legal_moves(&self, square: Square) -> Option<Vec<ChessMove>> {
        if let Some(piece) = &self.get(square) {
            match piece.kind {
                crate::chess::PieceKind::Knight => {
                    // corners only two directions
                    // middle of the board can go in 4 direction
                    if square.is_corner() {
                        // corner
                    }
                    Some(vec![])
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
