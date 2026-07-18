use crate::{
    board::Board,
    chess::{Color, Piece, PieceKind},
};

pub fn generate_view(piece: &Piece) -> char {
    match piece.color {
        Color::White => match piece.kind {
            PieceKind::King => '\u{2654}',
            PieceKind::Queen => '\u{2655}',
            PieceKind::Rook => '\u{2656}',
            PieceKind::Bishop => '\u{2657}',
            PieceKind::Knight => '\u{2658}',
            PieceKind::Pawn => '\u{2659}',
        },
        Color::Black => match piece.kind {
            PieceKind::King => '\u{265A}',
            PieceKind::Queen => '\u{265B}',
            PieceKind::Rook => '\u{265C}',
            PieceKind::Bishop => '\u{265D}',
            PieceKind::Knight => '\u{265E}',
            PieceKind::Pawn => '\u{265F}',
        },
    }
}
pub fn renderer(board: &Board) {
    print!("   ");
    if let Some(rank) = board.board.first() {
        for index in 0..rank.len() {
            let letter = (b'a' + index as u8) as char;
            print!("{letter:<4}"); // < left aligns the value, 4 gives a minimum width of 4
        }
    }
    println!(); //new line
    for (rank, pieces) in board.board.iter().enumerate().rev() {
        print!("{:<4}", rank + 1); // < left aligns the value, 4 gives a minimum width of 4
        for piece in pieces {
            if let Some(piece_view) = piece {
                let unicode = generate_view(piece_view);
                print!("{unicode:<4}");
            } else {
                print!("{:4}", "")
            }
        }
        println!(); //new line
    }
}

#[cfg(test)]
mod tests {
    use super::generate_view;
    use crate::chess::{Color, Piece, PieceKind};

    #[test]
    fn generate_view_renders_all_pieces() {
        let cases = [
            (Color::White, PieceKind::King, '♔'),
            (Color::White, PieceKind::Queen, '♕'),
            (Color::White, PieceKind::Rook, '♖'),
            (Color::White, PieceKind::Bishop, '♗'),
            (Color::White, PieceKind::Knight, '♘'),
            (Color::White, PieceKind::Pawn, '♙'),
            (Color::Black, PieceKind::King, '♚'),
            (Color::Black, PieceKind::Queen, '♛'),
            (Color::Black, PieceKind::Rook, '♜'),
            (Color::Black, PieceKind::Bishop, '♝'),
            (Color::Black, PieceKind::Knight, '♞'),
            (Color::Black, PieceKind::Pawn, '♟'),
        ];

        for (color, kind, expected) in cases {
            let piece = Piece::new(color, kind);

            assert_eq!(
                generate_view(&piece),
                expected,
                "incorrect rendering for {color:?} {kind:?}"
            );
        }
    }
}
