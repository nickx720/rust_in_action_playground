use crate::chess::{Piece, Square};

struct Board {
    board: [[Option<Piece>; 8]; 8],
}
impl Board {
    fn empty(self) -> bool {
        self.board.iter().all(|row| row.iter().all(Option::is_none))
    }

    fn starting_positing() -> Option<Piece> {
        todo!()
    }
    fn piece_at(square: Square) {
        todo!()
    }

    fn place_piece(square: Square, piece: Piece) {
        todo!()
    }

    fn remove_piece(square: Square) {
        todo!()
    }
}
