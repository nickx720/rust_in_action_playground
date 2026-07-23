use crate::{
    board::Board,
    chess::{ChessMove, Color, Piece, PieceKind, Square},
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
pub fn parser(input: &str) -> Option<ChessMove> {
    let items = input
        .split_whitespace()
        .filter_map(|item| {
            if item.len() == 2 {
                if let [first, second] = *item
                    .split("")
                    .filter(|&item| item != "".to_string())
                    .collect::<Vec<&str>>()
                    .as_slice()
                {
                    match first.chars().next() {
                        Some(file @ 'a'..='h') => match second.parse::<u8>() {
                            Ok(rank @ 1..=8) => {
                                let file = file as u8 - 97;
                                Some(Square::new(file, rank))
                            }
                            _ => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Square>>();
    if items.len() == 2 {
        let (from, to) = (
            items.get(0).expect("from is not present"),
            items.get(1).expect("to is not present"),
        );
        return Some(ChessMove::new(*from, *to));
    }
    None
}
#[cfg(test)]
mod tests {
    use super::{generate_view, parser};
    use crate::chess::{Color, Piece, PieceKind};

    #[test]
    fn parser_accepts_valid_squares() {
        parser("e4 e8");
    }

    #[test]
    fn parser_rejects_invalid_square() {
        parser("e9");
    }

    #[test]
    fn parser_rejects_invalid_input_between_valid_squares() {
        assert!(parser("e2 invalid e4").is_none());
    }

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
