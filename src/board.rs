use crate::piece::*;
use crate::square::*;
use crate::zobrist;

pub struct Board {
    state: [Option<Piece>; 64],
    zkey: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [None; 64],
            zkey: 0u64,
        }
    }

    pub fn place(&mut self, s: Square, p: Piece) {
        assert_eq!(self.state[s.index()].replace(p), None);
        self.zkey ^= zobrist::PIECE[s.index()][p.index()];
    }

    pub fn remove(&mut self, s: Square) -> Piece {
        let p: Piece = self.state[s.index()].take().unwrap();
        self.zkey ^= zobrist::PIECE[s.index()][p.index()];
        p
    }

    pub fn key(&self) -> u64 {
        self.zkey
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_place_works() {
        let mut b: Board = Board::new();

        b.place(Square::at(0, 0), Piece::from_fen('Q').unwrap());
    }

    #[test]
    fn board_remove_works() {
        let mut b: Board = Board::new();

        b.place(Square::at(0, 0), Piece::from_fen('Q').unwrap());
        assert_eq!(b.remove(Square::at(0, 0)), Piece::from_fen('Q').unwrap());
    }

    #[test]
    fn board_zkey_sanity() {
        let mut b: Board = Board::new();

        b.place(Square::at(0, 0), Piece::from_fen('Q').unwrap());
        b.place(Square::at(0, 1), Piece::from_fen('B').unwrap());
        b.place(Square::at(0, 2), Piece::from_fen('n').unwrap());
        b.place(Square::at(0, 3), Piece::from_fen('k').unwrap());
        b.place(Square::at(0, 4), Piece::from_fen('p').unwrap());
        b.place(Square::at(0, 5), Piece::from_fen('P').unwrap());
        b.place(Square::at(0, 6), Piece::from_fen('N').unwrap());
        b.place(Square::at(0, 7), Piece::from_fen('b').unwrap());

        for i in 0..8 {
            b.remove(Square::at(0, i));
        }

        assert_eq!(b.key(), 0u64);
    }

    #[test]
    #[should_panic]
    fn board_bad_place_fails() {
        let mut b: Board = Board::new();

        b.place(Square::at(0, 0), Piece::from_fen('Q').unwrap());
        b.place(Square::at(0, 0), Piece::from_fen('Q').unwrap());
    }

    #[test]
    #[should_panic]
    fn board_bad_remove_fails() {
        let mut b: Board = Board::new();

        b.remove(Square::at(0, 0));
    }
}