use crate::{board::Board, ui::renderer};

mod board;
mod chess;
mod moves;
mod ui;
fn main() {
    let board = Board::new();
    renderer(&board);
}
