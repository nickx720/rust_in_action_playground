# Rust Chess


## Challenge

Build a playable chess game in stages:

1. Represent and display an 8×8 chessboard with every piece in its initial
   position.
2. Generate and validate moves for each piece based on its type, position, and
   surrounding pieces.
3. Alternate between a human player and a computer player, rejecting illegal
   human moves. Initially, the computer may choose randomly from its legal
   moves.
4. Detect when the game has been won and end it.
5. Build a stronger chess engine by generating a game tree, searching several
   moves ahead, and pruning paths that do not need to be evaluated.

Useful search algorithms include minimax, alpha-beta pruning, and Monte Carlo
tree search.

## Suggested requirements

- Model the board, colors, pieces, positions, and game state.
- Generate legal moves for pawns, knights, bishops, rooks, queens, and kings.
- Prevent moves that leave the current player's king in check.
- Support captures and alternating turns.
- Detect checkmate and terminate the game.
- Cover move generation and validation with unit tests.

This plan intentionally excludes castling, en passant, promotion, stalemate,
draw conditions, advanced engine optimizations, and alternative board
representations. They are outside the scope of this exercise.

## Step-by-step implementation plan

Keep the chess rules independent from terminal input/output and computer-player
logic. A useful initial module layout is:

```text
src/
├── main.rs       # Starts the application
├── chess.rs      # Shared chess types
├── board.rs      # Board storage and setup
├── moves.rs      # Move generation and validation
├── game.rs       # Turns, game state, and move application
├── ui.rs         # Terminal rendering and input
└── engine.rs     # Computer-player search
```

Do not create every module immediately. Add each one when its milestone starts.

### 1. Define the chess vocabulary

Create types for:

- `Color`: `White` or `Black`
- `PieceKind`: pawn, knight, bishop, rook, queen, or king
- `Piece`: a color and piece kind
- `Square`: a board coordinate
- `ChessMove`: a source square and destination square

Start by representing a square as zero-based `file` and `rank` values from 0 to
7. Add conversion helpers for algebraic coordinates such as `e2`.

Tests:

- `a1` converts to `(0, 0)`.
- `h8` converts to `(7, 7)`.
- Coordinates outside the board are rejected.

Done when the domain types compile and coordinate conversion is tested.

### 2. Represent and initialize the board

Create a `Board` that stores 64 squares. A simple starting representation is:

```rust
struct Board {
    squares: [[Option<Piece>; 8]; 8],
}
```

Implement:

- An empty board
- The standard starting position
- Reading a square
- Placing and removing a piece

Tests:

- The starting board contains 32 pieces.
- White's king is on `e1`; Black's king is on `e8`.
- Pawns and back-rank pieces have the correct colors and positions.

Done when the starting position can be inspected entirely through `Board`
methods.

### 3. Render the board

Add a terminal renderer that prints all 64 squares, rank and file labels, and a
distinct symbol for each piece. Keep rendering read-only: it receives a
`&Board` and must not change game state.

Done when `cargo run` displays the standard starting position correctly.

### 4. Parse player moves

Accept a small input format such as:

```text
e2 e4
```

Convert it into a `ChessMove`. Reject malformed or out-of-bounds input without
panicking. At this point, do not decide whether the move is legal.

Tests:

- `e2 e4` parses successfully.
- Missing, extra, and invalid coordinates return errors.

Done when syntax validation and chess-rule validation are separate operations.

### 5. Generate pseudo-legal moves

Pseudo-legal moves follow a piece's movement rules but may leave its own king
in check. Implement one piece at a time in this order:

1. Knight — fixed jumps and no path traversal
2. Rook — horizontal and vertical sliding
3. Bishop — diagonal sliding
4. Queen — rook and bishop movement combined
5. King — one square in any direction
6. Pawn — direction, initial two-square move, and diagonal captures

For every piece, cover:

- Board edges
- Friendly pieces blocking movement
- Enemy pieces being capturable
- Sliding pieces stopping after the first occupied square

Use small custom board positions in tests instead of the full starting board.

Done when `pseudo_legal_moves(square, board)` works for all six piece types.

### 6. Apply moves

Add a method that moves a piece from its source to destination and handles a
capture. Return an error if the source is empty or the move cannot be applied.

Consider returning a new board or recording enough information to undo a move;
the search engine will eventually need to explore many hypothetical positions.

Tests:

- A normal move clears the source and fills the destination.
- A capture replaces the opposing piece.
- The original state remains recoverable if using make/unmake moves.

Done when moves can be applied without involving terminal input.

### 7. Detect attacks and check

Implement:

- Finding a color's king
- Determining whether a square is attacked by the opponent
- Determining whether a color is in check

Be careful with pawns: their attack squares differ from their forward movement.

Tests:

- Each piece type can give check.
- A blocked sliding piece does not give check.
- Pawn attacks use the correct direction.

Done when `is_in_check(color, board)` handles focused test positions.

### 8. Filter for fully legal moves

For each pseudo-legal move:

1. Apply it to a temporary board.
2. Check whether the moving side's king is attacked.
3. Keep the move only if the king is safe.

Reject moves made by the wrong color. This step creates the legal-move API that
the UI, game loop, and engine should all share.

Tests:

- A pinned piece cannot expose its king.
- A king cannot move onto an attacked square.
- A player in check must make a move that resolves the check.

Done when all accepted moves preserve the moving side's king.

### 9. Build the game loop

Create `Game` to own:

- The current board
- The side to move
- The current game status

The terminal loop should:

1. Display the board.
2. Read and parse a move.
3. Verify that it is in the legal-move list.
4. Apply it.
5. Switch turns.
6. Repeat.

Done when two humans can play using the terminal and invalid moves are rejected
without ending the program.

### 10. Detect game-ending states

After each move, generate all legal moves for the next player:

- In check with no legal moves: checkmate and the current player wins.
- Otherwise: continue the game.

Report the result and stop the game loop.

Tests:

- A known checkmate position is detected.
- A check with an available escape does not end the game.

Done when the game terminates correctly on checkmate.

### 11. Add a random computer player

Define a computer-player interface that accepts a position and returns a legal
move. The first implementation can select one move randomly.

Done when a human can complete a game against the computer.

### 12. Add position evaluation

Score a position from one player's perspective. Begin with material values:

- Pawn: 100
- Knight: 320
- Bishop: 330
- Rook: 500
- Queen: 900
- King: treated as invaluable

Tests should verify that winning material improves the correct side's score and
that mirrored positions produce opposite scores.

Done when the evaluator produces consistent, deterministic scores.

### 13. Implement search

Build the engine incrementally:

1. One-ply search: choose the move with the best immediate evaluation.
2. Negamax or minimax: search alternating turns to a fixed depth.
3. Alpha-beta pruning: skip branches that cannot affect the result.

Keep search independent of the terminal UI. Add checkmate scores that are more
important than any material score.

Done when the computer chooses legal moves within a predictable time limit and
can find simple tactical wins.

## Where to start now

Work only on steps 1 and 2 first:

1. Move the default `main.rs` code aside.
2. Create `chess.rs` with `Color`, `PieceKind`, `Piece`, `Square`, and
   `ChessMove`.
3. Test algebraic coordinate parsing.
4. Create `board.rs` with `Board::empty()` and `Board::starting_position()`.
5. Test the initial piece count and key starting squares.
6. Commit that working foundation before adding rendering or move rules.

Run `cargo test` after every small behavior is added. This keeps failures close
to the code that introduced them and prevents chess-rule bugs from becoming
entangled with input/output or search.
