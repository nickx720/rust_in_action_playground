#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind,
}
impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Self {
        Self { color, kind }
    }
}

// A square uses chess coordinates: `file` is the a–h column and `rank` is the
// 1–8 row, stored internally as zero-based values from 0 to 7.
#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }
}
// TODO: Derive `Debug`, `PartialEq`, and `Eq` for `ChessMove` (and
// `PartialEq`/`Eq` for `Square`) so parser tests can compare moves directly.
pub struct ChessMove {
    from: Square,
    to: Square,
}
impl ChessMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChessMove, Square};
    use crate::ui::parser;

    fn assert_moves_equal(actual: ChessMove, expected: ChessMove) {
        assert_eq!(actual.from.file, expected.from.file);
        assert_eq!(actual.from.rank, expected.from.rank);
        assert_eq!(actual.to.file, expected.to.file);
        assert_eq!(actual.to.rank, expected.to.rank);
    }

    #[test]
    fn parser_produces_expected_chess_move() {
        let actual = parser("e2 e4").expect("valid move should be parsed");
        let expected = ChessMove::new(Square::new(4, 1), Square::new(4, 3));

        assert_moves_equal(actual, expected);
    }

    #[test]
    fn parser_produces_zero_based_squares() {
        let actual = parser("a1 h8").expect("valid move should be parsed");
        let expected = ChessMove::new(Square::new(0, 0), Square::new(7, 7));

        assert_moves_equal(actual, expected);
    }
}
