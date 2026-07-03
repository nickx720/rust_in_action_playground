#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    color: Color,
    kind: PieceKind,
}

// A square uses chess coordinates: `file` is the a–h column and `rank` is the
// 1–8 row, stored internally as zero-based values from 0 to 7.
pub struct Square {
    file: u8,
    rank: u8,
}

struct ChessMove {
    from: Square,
    to: Square,
}
