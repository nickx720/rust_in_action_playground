use crate::{
    board::Board,
    chess::{ChessMove, Piece, Square},
};

impl Board {
    // codex resume 019fbc54-c741-78b2-a33e-90ab75d843f2
    pub fn get(&self, square: Square) -> Option<Piece> {
        self.board[square.rank as usize][square.file as usize]
    }
    pub fn pseudo_legal_moves(&self, square: Square) -> Option<Vec<ChessMove>> {
        if let Some(piece) = &self.get(square) {
            match piece.kind {
                crate::chess::PieceKind::Knight => {
                    let mut possible_moves = vec![];
                    let offsets = [
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ];

                    for (file_offset, rank_offset) in offsets {
                        let Some(file) = square.file.checked_add_signed(file_offset) else {
                            continue;
                        };
                        let Some(rank) = square.rank.checked_add_signed(rank_offset) else {
                            continue;
                        };

                        if file < 8 && rank < 8 {
                            possible_moves.push(ChessMove::new(square, Square::new(file, rank)));
                        }
                    }
                    Some(possible_moves)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::chess::{Color, PieceKind};

    fn empty_board() -> Board {
        Board {
            board: [[None; 8]; 8],
        }
    }

    fn assert_moves_match(mut actual: Vec<ChessMove>, mut expected: Vec<ChessMove>) {
        let coordinates = |chess_move: &ChessMove| {
            (
                chess_move.from.file,
                chess_move.from.rank,
                chess_move.to.file,
                chess_move.to.rank,
            )
        };

        actual.sort_by_key(&coordinates);
        expected.sort_by_key(&coordinates);

        assert_eq!(actual, expected);
    }

    #[test]
    fn psuedo_legal_moves_accepts_middle_knight_moves() {
        let mut board = empty_board();
        let from = Square::new(3, 3);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Knight));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a knight should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(1, 2)),
            ChessMove::new(from, Square::new(1, 4)),
            ChessMove::new(from, Square::new(2, 1)),
            ChessMove::new(from, Square::new(2, 5)),
            ChessMove::new(from, Square::new(4, 1)),
            ChessMove::new(from, Square::new(4, 5)),
            ChessMove::new(from, Square::new(5, 2)),
            ChessMove::new(from, Square::new(5, 4)),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Knight));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a knight should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(1, 2)),
            ChessMove::new(from, Square::new(2, 1)),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves_with_opponents() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        let first_opponent = Square::new(1, 2);
        let second_opponent = Square::new(2, 1);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Knight));
        board.place_piece(first_opponent, Piece::new(Color::Black, PieceKind::Pawn));
        board.place_piece(second_opponent, Piece::new(Color::Black, PieceKind::Bishop));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a knight should be able to capture opposing pieces");
        let expected = vec![
            ChessMove::new(from, first_opponent),
            ChessMove::new(from, second_opponent),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn pseudo_legal_moves_rejects_friendly_occupied_destinations() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        let friendly_destination = Square::new(1, 2);
        let opponent_destination = Square::new(2, 1);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Knight));
        board.place_piece(
            friendly_destination,
            Piece::new(Color::White, PieceKind::Pawn),
        );
        board.place_piece(
            opponent_destination,
            Piece::new(Color::Black, PieceKind::Bishop),
        );

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a knight should have pseudo-legal moves");
        let expected = vec![ChessMove::new(from, opponent_destination)];

        assert_moves_match(moves, expected);
    }
}
