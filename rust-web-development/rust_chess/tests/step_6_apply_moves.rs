// Black-box tests derived from README step 6. Import the modules here because
// this binary crate does not expose a library. No implementation edits needed.
#[path = "../src/board.rs"]
mod board;
#[path = "../src/chess.rs"]
mod chess;
#[path = "../src/moves.rs"]
mod moves;

use board::Board;
use chess::{Color, Piece, PieceKind, Square};

fn empty_board() -> Board {
    Board {
        board: [[None; 8]; 8],
    }
}

fn assert_same_piece(actual: Option<Piece>, expected: Option<Piece>) {
    match (actual, expected) {
        (None, None) => {}
        (Some(actual), Some(expected)) => {
            assert_eq!(
                std::mem::discriminant(&actual.color),
                std::mem::discriminant(&expected.color)
            );
            assert_eq!(
                std::mem::discriminant(&actual.kind),
                std::mem::discriminant(&expected.kind)
            );
        }
        _ => panic!("square occupancy differs from the expected position"),
    }
}

// Check every square, including bystanders, rather than only the endpoints.
fn assert_position(board: &Board, pieces: &[(u8, u8, Color, PieceKind)]) {
    for rank in 0..8 {
        for file in 0..8 {
            let expected = pieces
                .iter()
                .find(|&&(f, r, _, _)| f == file && r == rank)
                .map(|&(_, _, color, kind)| Piece::new(color, kind));
            assert_same_piece(board.get(Square::new(file, rank)), expected);
        }
    }
}

#[test]
fn step_6_normal_moves_clear_source_and_fill_destination_for_every_piece_and_color() {
    for color in [Color::White, Color::Black] {
        for kind in [
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Rook,
            PieceKind::Queen,
            PieceKind::King,
        ] {
            let (from, to) = match kind {
                PieceKind::Pawn => match color {
                    Color::White => ((3, 3), (3, 4)),
                    Color::Black => ((3, 3), (3, 2)),
                },
                PieceKind::Knight => ((3, 3), (4, 5)),
                PieceKind::Bishop => ((3, 3), (5, 5)),
                PieceKind::Rook => ((3, 3), (3, 6)),
                PieceKind::Queen => ((3, 3), (6, 6)),
                PieceKind::King => ((3, 3), (4, 4)),
            };
            let mut original = empty_board();
            original.place_piece(Square::new(from.0, from.1), Piece::new(color, kind));
            let moved = original.apply_move(Square::new(from.0, from.1), Square::new(to.0, to.1));
            assert_position(&moved, &[(to.0, to.1, color, kind)]);
            assert_position(&original, &[(from.0, from.1, color, kind)]);
        }
    }
}

#[test]
fn step_6_capture_replaces_opponent_and_preserves_original_and_bystanders() {
    for (moving, opponent) in [(Color::White, Color::Black), (Color::Black, Color::White)] {
        let mut original = empty_board();
        original.place_piece(Square::new(0, 0), Piece::new(moving, PieceKind::Rook));
        original.place_piece(Square::new(0, 7), Piece::new(opponent, PieceKind::Bishop));
        original.place_piece(Square::new(7, 2), Piece::new(moving, PieceKind::Knight));
        original.place_piece(Square::new(6, 5), Piece::new(opponent, PieceKind::Pawn));

        let moved = original.apply_move(Square::new(0, 0), Square::new(0, 7));
        assert_position(
            &moved,
            &[
                (0, 7, moving, PieceKind::Rook),
                (7, 2, moving, PieceKind::Knight),
                (6, 5, opponent, PieceKind::Pawn),
            ],
        );
        assert_position(
            &original,
            &[
                (0, 0, moving, PieceKind::Rook),
                (0, 7, opponent, PieceKind::Bishop),
                (7, 2, moving, PieceKind::Knight),
                (6, 5, opponent, PieceKind::Pawn),
            ],
        );
    }
}

#[test]
fn step_6_pawn_diagonal_capture_moves_the_pawn_for_both_colors() {
    for (moving, opponent, destination_rank) in [
        (Color::White, Color::Black, 4),
        (Color::Black, Color::White, 2),
    ] {
        let mut original = empty_board();
        original.place_piece(Square::new(3, 3), Piece::new(moving, PieceKind::Pawn));
        original.place_piece(
            Square::new(4, destination_rank),
            Piece::new(opponent, PieceKind::Queen),
        );
        let moved = original.apply_move(Square::new(3, 3), Square::new(4, destination_rank));
        assert_position(&moved, &[(4, destination_rank, moving, PieceKind::Pawn)]);
    }
}

#[test]
fn step_6_moves_between_board_corners_work_in_both_directions() {
    for (from, to) in [
        ((0, 0), (7, 0)),
        ((7, 0), (7, 7)),
        ((7, 7), (0, 7)),
        ((0, 7), (0, 0)),
    ] {
        let mut original = empty_board();
        original.place_piece(
            Square::new(from.0, from.1),
            Piece::new(Color::White, PieceKind::Rook),
        );
        let moved = original.apply_move(Square::new(from.0, from.1), Square::new(to.0, to.1));
        assert_position(&moved, &[(to.0, to.1, Color::White, PieceKind::Rook)]);
    }
}

#[test]
fn step_6_successive_moves_preserve_each_previous_position() {
    let mut original = empty_board();
    original.place_piece(Square::new(0, 0), Piece::new(Color::White, PieceKind::Rook));
    original.place_piece(Square::new(7, 7), Piece::new(Color::Black, PieceKind::Rook));
    let first = original.apply_move(Square::new(0, 0), Square::new(0, 7));
    let second = first.apply_move(Square::new(7, 7), Square::new(0, 7));

    assert_position(
        &original,
        &[
            (0, 0, Color::White, PieceKind::Rook),
            (7, 7, Color::Black, PieceKind::Rook),
        ],
    );
    assert_position(
        &first,
        &[
            (0, 7, Color::White, PieceKind::Rook),
            (7, 7, Color::Black, PieceKind::Rook),
        ],
    );
    assert_position(&second, &[(0, 7, Color::Black, PieceKind::Rook)]);
}

// The current signature returns Board. This adapter keeps the behavioral tests
// runnable while making the README's missing error contract a test failure.
// If the API later returns Result, the same assertion checks for an actual Err.
trait ErrorOutcome {
    fn assert_error(self);
}

impl ErrorOutcome for Board {
    fn assert_error(self) {
        panic!("README step 6 requires an error; apply_move returned Board instead of Result");
    }
}

impl<T, E> ErrorOutcome for Result<T, E> {
    fn assert_error(self) {
        assert!(self.is_err(), "an inapplicable move must return an error");
    }
}

#[test]
fn step_6_empty_source_to_empty_destination_returns_error() {
    empty_board()
        .apply_move(Square::new(2, 2), Square::new(2, 3))
        .assert_error();
}

#[test]
fn step_6_empty_source_to_occupied_destination_returns_error() {
    let mut board = empty_board();
    board.place_piece(Square::new(2, 3), Piece::new(Color::Black, PieceKind::Pawn));
    board
        .apply_move(Square::new(2, 2), Square::new(2, 3))
        .assert_error();
}

#[test]
fn step_6_same_source_and_destination_returns_error() {
    let mut board = empty_board();
    board.place_piece(Square::new(3, 3), Piece::new(Color::White, PieceKind::Rook));
    board
        .apply_move(Square::new(3, 3), Square::new(3, 3))
        .assert_error();
}
