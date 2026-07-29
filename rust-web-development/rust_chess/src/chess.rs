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
pub(crate) struct Square {
    pub(crate) file: u8,
    pub(crate) rank: u8,
}

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }
}

#[derive(Debug)]
pub struct ChessMove {
    pub from: Square,
    pub to: Square,
}
impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
        self.from.file == other.from.file && self.to.rank == other.to.rank
    }
}
impl Eq for ChessMove {}
impl ChessMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}
