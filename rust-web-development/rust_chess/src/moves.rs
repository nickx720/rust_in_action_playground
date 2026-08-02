use crate::{
    board::Board,
    chess::{ChessMove, Piece, Square},
};
// codex resume 019fbc54-c741-78b2-a33e-90ab75d843f2
impl Board {
    pub fn get(&self, square: Square) -> Option<Piece> {
        self.board[square.file as usize][square.rank as usize]
    }
    pub fn pseudo_legal_moves(&self, square: Square) -> Option<Vec<ChessMove>> {
        if let Some(piece) = &self.get(square) {
            dbg!(piece);
            Some(vec![])
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
