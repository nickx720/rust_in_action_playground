use crate::{board::Board, chess::Square};
// codex resume 019fbc54-c741-78b2-a33e-90ab75d843f2
impl Board {
    pub fn pseudo_legal_moves(&self, square: Square) -> Vec<Square> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn psuedo_legal_moves_accepts_middle_knight_moves() {
        todo!()
    }
    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves() {
        todo!()
    }
    #[test]
    fn psuedo_legal_moves_accepts_corner_knight_moves_with_opponents() {
        todo!()
    }
}
