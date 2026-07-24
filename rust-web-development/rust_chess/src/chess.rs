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
#[derive(Clone, Copy, Debug)]
pub struct ChessMove {
    from: Square,
    to: Square,
}
impl ChessMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}
