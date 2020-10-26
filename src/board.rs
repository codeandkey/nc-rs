use crate::attacks;
use crate::bitboard;
use crate::piece::*;
use crate::square::*;
use crate::zobrist;

pub struct Board {
    state: [Option<Piece>; 64],
    zkey: u64,
    ad: [[u64; 64]; 2],
    occ: u64,
    color: [u64; 2],
    piece: [u64; 6],
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [None; 64],
            zkey: 0u64,
            ad: [[0; 64]; 2],
            occ: 0u64,
            color: [0u64; 2],
            piece: [0u64; 6],
        }
    }

    pub fn place(&mut self, s: Square, p: Piece) {
        assert!(!self.state[s.index()].is_some());

        /* Find pieces that require an A/D recalculation */

        let mut umask: u64 = 0u64;
        umask |= (self.piece[Type::QUEEN as usize] | self.piece[Type::BISHOP as usize]) & attacks::bishop(s, self.occ);
        umask |= (self.piece[Type::QUEEN as usize] | self.piece[Type::ROOK as usize]) & attacks::rook(s, self.occ);

        /* Remove attacks from required updates */
        bitboard::for_each(umask, |ts| {
            self.remove_attacks(ts);
        });

        /* Place piece and update occs */
        self.state[s.index()].replace(p);
        self.zkey ^= zobrist::PIECE[s.index()][p.index()];

        let mask: u64 = s.mask();
        self.occ ^= mask;
        self.color[p.get_color() as usize] ^= mask;
        self.piece[p.get_type() as usize] ^= mask;

        /* Add attacks from piece being placed */
        self.add_attacks(s);

        /* Re-add attacks from pieces that needed update */
        bitboard::for_each(umask, |ts| {
            self.add_attacks(ts);
        });
    }

    pub fn remove(&mut self, s: Square) -> Piece {
        assert!(self.state[s.index()].is_some());

        /* Find pieces that require an A/D recalculation */

        let mut umask: u64 = 0u64;
        umask |= (self.piece[Type::QUEEN as usize] | self.piece[Type::BISHOP as usize]) & attacks::bishop(s, self.occ);
        umask |= (self.piece[Type::QUEEN as usize] | self.piece[Type::ROOK as usize]) & attacks::rook(s, self.occ);

        /* Remove attacks from required updates */
        bitboard::for_each(umask, |ts| {
            self.remove_attacks(ts);
        });

        /* Remove attacks from piece being removed */
        self.remove_attacks(s);

        /* Remove piece and update occs */
        let p: Piece = self.state[s.index()].take().unwrap();
        self.zkey ^= zobrist::PIECE[s.index()][p.index()];

        /* Drop occupancies */
        let mask: u64 = s.mask();
        self.occ ^= mask;
        self.color[p.get_color() as usize] ^= mask;
        self.piece[p.get_type() as usize] ^= mask;

        /* Re-add attacks from pieces that needed update */
        bitboard::for_each(umask, |ts| {
            self.add_attacks(ts);
        });

        p
    }

    pub fn key(&self) -> u64 {
        self.zkey
    }

    pub fn add_attacks(&mut self, s: Square) {
        let p = self.state[s.index()].unwrap();
        let c = p.get_color();
        let tp = p.get_type();

        let att = match tp {
            Type::PAWN => attacks::pawn(c, s),
            Type::BISHOP => attacks::bishop(s, self.occ),
            Type::KNIGHT => attacks::knight(s),
            Type::ROOK => attacks::rook(s, self.occ),
            Type::QUEEN => attacks::queen(s, self.occ),
            Type::KING => attacks::king(s),
        };

        bitboard::for_each(att, |s| {
            self.ad[c as usize][s.index()] += 1;
        });
    }

    pub fn remove_attacks(&mut self, s: Square) {
        let p = self.state[s.index()].unwrap();
        let c = p.get_color();
        let tp = p.get_type();

        let att = match tp {
            Type::PAWN => attacks::pawn(c, s),
            Type::BISHOP => attacks::bishop(s, self.occ),
            Type::KNIGHT => attacks::knight(s),
            Type::ROOK => attacks::rook(s, self.occ),
            Type::QUEEN => attacks::queen(s, self.occ),
            Type::KING => attacks::king(s),
        };

        bitboard::for_each(att, |s| {
            self.ad[c as usize][s.index()] -= 1;
        });
    }

    pub fn piece_occ(&self, t: Type) -> u64 {
        self.piece[t as usize]
    }

    pub fn get_ad(&self) -> [[u64; 64]; 2] {
        self.ad
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
