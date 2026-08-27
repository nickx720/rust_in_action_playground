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
                            let pos_square = Square::new(file, rank);
                            if let Some(piece_at_position) = self.get(pos_square)
                                && piece_at_position.color == piece.color
                            {
                                continue;
                            }
                            possible_moves.push(ChessMove::new(square, pos_square));
                        }
                    }
                    Some(possible_moves)
                }
                crate::chess::PieceKind::King => {
                    let mut possible_moves = vec![];
                    let offsets = [
                        (1, 1), //diagonal
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                        (1, 0), // not diagonal
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                    ];

                    for (file_offset, rank_offset) in offsets {
                        let Some(file) = square.file.checked_add_signed(file_offset) else {
                            continue;
                        };
                        let Some(rank) = square.rank.checked_add_signed(rank_offset) else {
                            continue;
                        };

                        if file < 8 && rank < 8 {
                            let pos_square = Square::new(file, rank);
                            if let Some(piece_at_position) = self.get(pos_square)
                                && piece_at_position.color == piece.color
                            {
                                continue;
                            }
                            possible_moves.push(ChessMove::new(square, pos_square));
                        }
                    }
                    Some(possible_moves)
                }
                crate::chess::PieceKind::Bishop => {
                    // can move diagonally till it comes across a friendly piece
                    // can occupy a square if occupied by the opponent piece
                    // can move in any direction
                    let mut possible_moves: Vec<ChessMove> = vec![];
                    let offsets = [
                        (1, 1), //diagonal
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                    ];
                    for (file_offset, rank_offset) in offsets {
                        // loop through to edge conditions
                        let (mut file, mut rank) = (square.file, square.rank);
                        loop {
                            let Some(file_add) = file.checked_add_signed(file_offset) else {
                                continue;
                            };
                            let Some(rank_add) = rank.checked_add_signed(rank_offset) else {
                                continue;
                            };
                            dbg!(file, rank, file_add, rank_add);
                            if file < 8 && rank < 8 && file_add > 0 && rank_add > 0 {
                                let pos_square = Square::new(file_add, rank_add);
                                if let Some(piece_at_position) = self.get(pos_square) {
                                    if piece_at_position.color == piece.color {
                                        break;
                                    }
                                    if piece_at_position.color != piece.color {
                                        possible_moves.push(ChessMove::new(square, pos_square));
                                        break;
                                    }
                                }
                                possible_moves.push(ChessMove::new(square, pos_square));
                                file = file_add;
                                rank = rank_add;
                            } else {
                                break;
                            }
                        }
                        dbg!(&possible_moves);
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

    #[test]
    fn psuedo_legal_moves_accepts_middle_king_moves() {
        let mut board = empty_board();
        let from = Square::new(3, 3);
        board.place_piece(from, Piece::new(Color::White, PieceKind::King));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a king should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(2, 2)),
            ChessMove::new(from, Square::new(2, 3)),
            ChessMove::new(from, Square::new(2, 4)),
            ChessMove::new(from, Square::new(3, 2)),
            ChessMove::new(from, Square::new(3, 4)),
            ChessMove::new(from, Square::new(4, 2)),
            ChessMove::new(from, Square::new(4, 3)),
            ChessMove::new(from, Square::new(4, 4)),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn psuedo_legal_moves_accepts_corner_king_moves() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        board.place_piece(from, Piece::new(Color::White, PieceKind::King));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a king should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(0, 1)),
            ChessMove::new(from, Square::new(1, 0)),
            ChessMove::new(from, Square::new(1, 1)),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn psuedo_legal_moves_accepts_corner_king_moves_with_opponents() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        let first_opponent = Square::new(0, 1);
        let second_opponent = Square::new(1, 0);
        let third_opponent = Square::new(1, 1);
        board.place_piece(from, Piece::new(Color::White, PieceKind::King));
        board.place_piece(first_opponent, Piece::new(Color::Black, PieceKind::Pawn));
        board.place_piece(second_opponent, Piece::new(Color::Black, PieceKind::Bishop));
        board.place_piece(third_opponent, Piece::new(Color::Black, PieceKind::Knight));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a king should be able to capture opposing pieces");
        let expected = vec![
            ChessMove::new(from, first_opponent),
            ChessMove::new(from, second_opponent),
            ChessMove::new(from, third_opponent),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn pseudo_legal_king_moves_reject_friendly_occupied_destinations() {
        let mut board = empty_board();
        let from = Square::new(0, 0);
        let friendly_destination = Square::new(0, 1);
        let first_opponent = Square::new(1, 0);
        let second_opponent = Square::new(1, 1);
        board.place_piece(from, Piece::new(Color::White, PieceKind::King));
        board.place_piece(
            friendly_destination,
            Piece::new(Color::White, PieceKind::Pawn),
        );
        board.place_piece(first_opponent, Piece::new(Color::Black, PieceKind::Bishop));
        board.place_piece(second_opponent, Piece::new(Color::Black, PieceKind::Knight));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a king should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, first_opponent),
            ChessMove::new(from, second_opponent),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn pseudo_legal_moves_accepts_middle_bishop_moves() {
        let mut board = empty_board();
        let from = Square::new(3, 3);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Bishop));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a bishop should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(0, 0)),
            ChessMove::new(from, Square::new(1, 1)),
            ChessMove::new(from, Square::new(2, 2)),
            ChessMove::new(from, Square::new(4, 4)),
            ChessMove::new(from, Square::new(5, 5)),
            ChessMove::new(from, Square::new(6, 6)),
            ChessMove::new(from, Square::new(7, 7)),
            ChessMove::new(from, Square::new(0, 6)),
            ChessMove::new(from, Square::new(1, 5)),
            ChessMove::new(from, Square::new(2, 4)),
            ChessMove::new(from, Square::new(4, 2)),
            ChessMove::new(from, Square::new(5, 1)),
            ChessMove::new(from, Square::new(6, 0)),
        ];

        assert_moves_match(moves, expected);
    }

    //    #[test]
    //    fn pseudo_legal_moves_accepts_corner_bishop_moves() {
    //        let mut board = empty_board();
    //        let from = Square::new(0, 0);
    //        board.place_piece(from, Piece::new(Color::White, PieceKind::Bishop));
    //
    //        let moves = board
    //            .pseudo_legal_moves(from)
    //            .expect("a bishop should have pseudo-legal moves");
    //        let expected = (1..8)
    //            .map(|coordinate| ChessMove::new(from, Square::new(coordinate, coordinate)))
    //            .collect();
    //
    //        assert_moves_match(moves, expected);
    //    }

    #[test]
    fn pseudo_legal_bishop_moves_capture_opponents_without_moving_past_them() {
        let mut board = empty_board();
        let from = Square::new(2, 2);
        let opponent = Square::new(4, 4);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Bishop));
        board.place_piece(opponent, Piece::new(Color::Black, PieceKind::Pawn));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a bishop should be able to capture an opposing piece");
        let expected = vec![
            ChessMove::new(from, Square::new(0, 0)),
            ChessMove::new(from, Square::new(1, 1)),
            ChessMove::new(from, Square::new(1, 3)),
            ChessMove::new(from, Square::new(0, 4)),
            ChessMove::new(from, Square::new(3, 1)),
            ChessMove::new(from, Square::new(4, 0)),
            ChessMove::new(from, Square::new(3, 3)),
            ChessMove::new(from, opponent),
        ];

        assert_moves_match(moves, expected);
    }

    #[test]
    fn pseudo_legal_bishop_moves_stop_before_friendly_pieces() {
        let mut board = empty_board();
        let from = Square::new(2, 2);
        let friendly_blocker = Square::new(4, 4);
        board.place_piece(from, Piece::new(Color::White, PieceKind::Bishop));
        board.place_piece(friendly_blocker, Piece::new(Color::White, PieceKind::Pawn));

        let moves = board
            .pseudo_legal_moves(from)
            .expect("a bishop should have pseudo-legal moves");
        let expected = vec![
            ChessMove::new(from, Square::new(0, 0)),
            ChessMove::new(from, Square::new(1, 1)),
            ChessMove::new(from, Square::new(1, 3)),
            ChessMove::new(from, Square::new(0, 4)),
            ChessMove::new(from, Square::new(3, 1)),
            ChessMove::new(from, Square::new(4, 0)),
            ChessMove::new(from, Square::new(3, 3)),
        ];

        assert_moves_match(moves, expected);
    }
}
