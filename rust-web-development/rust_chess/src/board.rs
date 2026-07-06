use crate::chess::{Piece, Square};

struct Board {
    board: [[Option<Piece>; 8]; 8],
}
impl Board {
    fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }
    fn empty(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(Option::is_none))
    }

    fn starting_positing(&self) -> Option<Piece> {
        (self.board.first().and_then(|inner| inner.first()))
            .copied()
            .expect("nested array must contain an item")
    }
    fn piece_at(&self, square: Square) {
        self.board
        todo!()
    }

    fn place_piece(square: Square, piece: Piece) {
        todo!()
    }

    fn remove_piece(square: Square) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn empty_board_is_empty() {
        let board = Board::new();

        assert!(board.empty());
    }

    #[test]
    fn starting_position_on_an_empty_board_is_none() {
        let board = Board::new();
        assert!(board.starting_positing().is_none());
    }
}
