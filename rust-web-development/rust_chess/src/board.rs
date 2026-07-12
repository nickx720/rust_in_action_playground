use crate::chess::{Piece, Square};

pub struct Board {
    board: [[Option<Piece>; 8]; 8],
}
impl Board {
    pub(crate) fn new() -> Self {
        Self {
            board: [[None; 8]; 8],
        }
    }
    fn empty(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(Option::is_none))
    }

    pub(crate) fn starting_positing(&self) -> Option<Piece> {
        (self.board.first().and_then(|inner| inner.first()))
            .copied()
            .expect("nested array must contain an item")
    }
    pub(crate) fn piece_at(&self, pos: (usize, usize)) -> Option<Piece> {
        self.board
            .get(pos.0)
            .and_then(|item| item.get(pos.1))
            .copied()
            .expect("Unable to find piece")
    }

    pub(crate) fn place_piece(&mut self, square: Square, piece: Piece) {
        self.board[square.rank as usize][square.file as usize] = Some(piece);
    }

    pub(crate) fn remove_piece(&mut self, square: Square) {
        self.board[square.rank as usize][square.file as usize] = None;
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use crate::chess::{Color, Piece, PieceKind, Square};

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

    #[test]
    fn placing_a_piece_makes_the_board_non_empty() {
        let mut board = Board::new();
        let king = Piece::new(Color::White, PieceKind::King);

        board.place_piece(Square::new(4, 0), king);

        assert!(!board.empty());
        assert_eq!(board.piece_at((0, 4)), Some(king));
    }

    #[test]
    fn place_piece_uses_rank_then_file_for_board_storage() {
        let mut board = Board::new();
        let bishop = Piece::new(Color::Black, PieceKind::Bishop);

        board.place_piece(Square::new(2, 5), bishop);

        assert_eq!(board.piece_at((5, 2)), Some(bishop));
        assert_eq!(board.piece_at((2, 5)), None);
    }

    #[test]
    fn removing_a_piece_clears_its_square() {
        let mut board = Board::new();
        let rook = Piece::new(Color::White, PieceKind::Rook);
        let square = Square::new(7, 3);

        board.place_piece(square, rook);
        board.remove_piece(square);

        assert_eq!(board.piece_at((3, 7)), None);
        assert!(board.empty());
    }

    #[test]
    fn removing_an_empty_square_is_a_noop() {
        let mut board = Board::new();

        board.remove_piece(Square::new(3, 3));

        assert!(board.empty());
    }

    #[test]
    fn placing_on_an_occupied_square_replaces_the_piece() {
        let mut board = Board::new();
        let square = Square::new(3, 3);
        let pawn = Piece::new(Color::White, PieceKind::Pawn);
        let queen = Piece::new(Color::Black, PieceKind::Queen);

        board.place_piece(square, pawn);
        board.place_piece(square, queen);

        assert_eq!(board.piece_at((3, 3)), Some(queen));
    }

    #[test]
    fn removing_one_piece_does_not_change_another_square() {
        let mut board = Board::new();
        let knight = Piece::new(Color::White, PieceKind::Knight);
        let queen = Piece::new(Color::Black, PieceKind::Queen);
        let knight_square = Square::new(1, 0);
        let queen_square = Square::new(3, 7);

        board.place_piece(knight_square, knight);
        board.place_piece(queen_square, queen);
        board.remove_piece(knight_square);

        assert_eq!(board.piece_at((0, 1)), None);
        assert_eq!(board.piece_at((7, 3)), Some(queen));
        assert!(!board.empty());
    }

    #[test]
    fn pieces_can_be_placed_on_both_board_boundaries() {
        let mut board = Board::new();
        let rook = Piece::new(Color::White, PieceKind::Rook);
        let king = Piece::new(Color::Black, PieceKind::King);

        board.place_piece(Square::new(0, 0), rook);
        board.place_piece(Square::new(7, 7), king);

        assert_eq!(board.piece_at((0, 0)), Some(rook));
        assert_eq!(board.piece_at((7, 7)), Some(king));
    }

    #[test]
    #[should_panic(expected = "Unable to find piece")]
    fn reading_beyond_the_board_panics() {
        let board = Board::new();

        let _ = board.piece_at((8, 0));
    }
}
