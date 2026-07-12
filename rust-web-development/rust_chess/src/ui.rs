use crate::board::Board;
pub fn renderer(board: &Board) {
    // Which rank and file is currently being displayed?
    //  - Is that square empty or occupied?
    //  - If occupied, which visible symbol represents that piece’s kind and color?
    //  - In which direction should ranks be displayed so White appears at the bottom?
    //  - Where should rank and file labels appear?
    //
    //  Think of it as a translation:
    //
    //  board state → visual representation → terminal output
    //
    //  Keep those stages conceptually separate. The renderer should not know how pieces arrived at their squares,
    //  whether a position is legal, or whose turn it is.
    todo!()
}
