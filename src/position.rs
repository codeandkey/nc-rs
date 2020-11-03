use crate::attacks;
use crate::bitboard;
use crate::board::*;
use crate::gmove::*;
use crate::piece::*;
use crate::square::*;
use crate::zobrist::*;

#[derive(Copy, Clone)]
pub enum Castling {
    QUEENSIDE = 0,
    KINGSIDE = 1,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    ep_target: Option<Square>,      /* En-passant target square */
    captured: Option<Piece>,        /* Piece captured */
    capture_square: Option<Square>, /* Square captured on */
    fm_number: u32,                 /* Move number */
    hm_clock: u32,                  /* Halfmove clock */
    castling: [[bool; 2]; 2],       /* Castling rights */
    last_move: Option<Move>,        /* Last move */
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Position {
    ply: Vec<State>,
    b: Board,
    ctm: Color,
}

mod movegen {
    use super::*;

    /* pawns on these masks can promote */
    pub const PAWN_PROMOTE_MASK: [u64; 2] = [bitboard::RANKS[6], bitboard::RANKS[1]];

    /* pawns on these masks can jump */
    pub const PAWN_JUMP_MASK: [u64; 2] = [bitboard::RANKS[1], bitboard::RANKS[6]];

    /* pawn move direction */
    pub const PAWN_DIRECTION: [i32; 2] = [Direction::NORTH as i32, Direction::SOUTH as i32];

    /* pawn left captures */
    pub const PAWN_LC_DIRECTION: [i32; 2] =
        [Direction::NORTHWEST as i32, Direction::SOUTHWEST as i32];

    /* pawn right captures */
    pub const PAWN_RC_DIRECTION: [i32; 2] =
        [Direction::NORTHEAST as i32, Direction::SOUTHEAST as i32];

    /* castle noattack masks */
    pub const CASTLE_ATT_MASKS: [[u64; 2]; 2] = [
        [
            bitboard::RANKS[0] & (bitboard::FILES[2] | bitboard::FILES[3] | bitboard::FILES[4]),
            bitboard::RANKS[0] & (bitboard::FILES[4] | bitboard::FILES[5] | bitboard::FILES[6]),
        ],
        [
            bitboard::RANKS[7] & (bitboard::FILES[2] | bitboard::FILES[3] | bitboard::FILES[4]),
            bitboard::RANKS[7] & (bitboard::FILES[4] | bitboard::FILES[5] | bitboard::FILES[6]),
        ],
    ];

    /* castle occ masks */
    pub const CASTLE_OCC_MASKS: [[u64; 2]; 2] = [
        [
            bitboard::RANKS[0] & (bitboard::FILES[1] | bitboard::FILES[2] | bitboard::FILES[3]),
            bitboard::RANKS[0] & (bitboard::FILES[5] | bitboard::FILES[6]),
        ],
        [
            bitboard::RANKS[7] & (bitboard::FILES[1] | bitboard::FILES[2] | bitboard::FILES[3]),
            bitboard::RANKS[7] & (bitboard::FILES[5] | bitboard::FILES[6]),
        ],
    ];
}

impl Position {
    pub fn new(fen: String) -> Option<Position> {
        let parts: Vec<&str> = fen.split(' ').collect();

        if parts.len() != 6 {
            error!(
                "Invalid number of FEN parts: expected 6, read {}",
                parts.len()
            );
            return None;
        }

        /* Parse board data */
        let ranks: Vec<&str> = parts[0].split('/').collect();

        if ranks.len() != 8 {
            error!(
                "Invalid number of FEN ranks: expected 8, read {}",
                ranks.len()
            );
            return None;
        }

        /* Parse individual ranks */
        let mut b = Board::new();

        for r in 0..8 {
            let mut f = 0;

            for c in ranks[r].chars() {
                if f >= 8 {
                    error!("Too many pieces in rank '{}'", r);
                    return None;
                }

                if c.is_digit(10) {
                    /* Skip files */
                    f += c.to_digit(10).unwrap();
                } else {
                    /* Place piece */
                    match Piece::from_fen(c) {
                        Some(p) => b.place(Square::at((7 - r) as usize, f as usize), p),
                        None => {
                            error!("Invalid piece character '{}' in FEN", c);
                            return None;
                        }
                    }

                    f += 1;
                }
            }
        }

        /* Parse color to move */
        if parts[1].len() != 1 {
            error!("Invalid color '{}' in FEN", parts[1]);
            return None;
        }

        let ctm = Color::from_fen(parts[1].chars().next().unwrap());

        if ctm.is_none() {
            error!("Invalid color '{}' in FEN", parts[1]);
            return None;
        }

        let ctm = ctm.unwrap();

        /* Parse castling rights */
        let mut rights = [[false; 2]; 2];

        for c in parts[2].chars() {
            match c {
                'Q' => rights[Color::WHITE as usize][Castling::QUEENSIDE as usize] = true,
                'K' => rights[Color::WHITE as usize][Castling::KINGSIDE as usize] = true,
                'q' => rights[Color::BLACK as usize][Castling::QUEENSIDE as usize] = true,
                'k' => rights[Color::BLACK as usize][Castling::KINGSIDE as usize] = true,
                _ => {} /* Just ignore invalid characters */
            }
        }

        /* Parse ep target */
        let ep_target = Square::from_uci(&parts[3].to_string());

        /* Parse hm clock */
        let hm_clock = parts[4].parse::<u32>();

        if hm_clock.is_err() {
            error!("Invalid halfmove clock '{}'", parts[4]);
            return None;
        }

        let hm_clock = hm_clock.unwrap();

        /* Parse move number */
        let fm_number = parts[5].parse::<u32>();

        if fm_number.is_err() {
            error!("Invalid move number '{}'", parts[5]);
            return None;
        }

        let fm_number = fm_number.unwrap();

        Some(Position {
            ply: vec![State {
                ep_target: ep_target,
                captured: None,
                capture_square: None,
                fm_number: fm_number,
                hm_clock: hm_clock,
                castling: rights,
                last_move: None,
            }],
            b: b,
            ctm: ctm,
        })
    }

    pub fn to_fen(&self) -> String {
        let mut output: String = self.b.to_fen_string();

        /* Add color to move */
        output.push_str(&format!(" {} ", self.ctm.to_fen()));

        /* Add castling */
        let top = self.ply.last().unwrap();

        if top.castling[Color::WHITE as usize][Castling::KINGSIDE as usize] {
            output.push('K');
        }

        if top.castling[Color::WHITE as usize][Castling::QUEENSIDE as usize] {
            output.push('Q');
        }

        if top.castling[Color::BLACK as usize][Castling::KINGSIDE as usize] {
            output.push('k');
        }

        if top.castling[Color::BLACK as usize][Castling::QUEENSIDE as usize] {
            output.push('q');
        }

        output.push(' ');

        /* Add ep target */
        output.push_str(&Square::to_str_withnull(&top.ep_target));

        /* Add halfmove clock */
        output.push_str(&format!(" {}", top.hm_clock));

        /* Add move number */
        output.push_str(&format!(" {}", top.fm_number));

        output
    }

    pub fn make_move(&mut self, m: Move) -> bool {
        //println!("making move {}, current ply = {}", m.to_uci(), self.ply.len());
        let last_state = self.ply.last().unwrap();
        let mut new_state = last_state.clone();

        /* Reset per-move fields */
        new_state.ep_target = None;
        new_state.captured = None;
        new_state.capture_square = None;
        new_state.last_move = Some(m.clone());
        new_state.hm_clock += 1;

        if self.ctm == Color::WHITE {
            new_state.fm_number += 1;
        }

        /* Test if move is capture */
        if self.b.color_occ(self.ctm.flip()) & m.dst().mask() != 0u64 {
            new_state.captured = Some(self.b.remove(m.dst()));
            new_state.capture_square = Some(m.dst());
        }

        /* Test for ep capture */
        if last_state.ep_target.is_some() {
            if self.b.piece_occ(Type::PAWN) & m.src().mask() != 0u64
                && m.dst() == last_state.ep_target.unwrap()
            {
                /* Remove captured piece at expected square */
                let ep_square = Square::from_index(
                    (last_state.ep_target.unwrap().index() as i32
                        - movegen::PAWN_DIRECTION[self.ctm as usize]) as usize,
                )
                .unwrap();

                new_state.captured = Some(self.b.remove(ep_square));
                new_state.capture_square = Some(ep_square);
            }
        }

        let mut castle_att_mask: u64 = 0u64;

        /* Test if move is castle */
        if self.b.piece_occ(Type::KING) & m.src().mask() != 0u64 {
            if (m.src().file() as i32 - m.dst().file() as i32).abs() > 1 {
                let castle_side = match m.dst().file() {
                    2 => Castling::QUEENSIDE,
                    6 => Castling::KINGSIDE,
                    a => panic!("Invalid castling file {}", a),
                };

                castle_att_mask =
                    movegen::CASTLE_ATT_MASKS[self.ctm as usize][castle_side as usize];

                /* Place the rook on the new square */
                match castle_side {
                    Castling::QUEENSIDE => {
                        let p = self.b.remove(Square::at(m.src().rank(), 0));
                        self.b.place(Square::at(m.src().rank(), 3), p);
                    }
                    Castling::KINGSIDE => {
                        let p = self.b.remove(Square::at(m.src().rank(), 7));
                        self.b.place(Square::at(m.src().rank(), 5), p);
                    }
                }
            }

            /* Revoke all castling rights on any king move */
            new_state.castling[self.ctm as usize] = [false, false];
        }

        /* Revoke castling rights on corner moves */
        if self.b.piece_occ(Type::ROOK) & m.src().mask() != 0u64 {
            match m.src().index() {
                0 => {
                    new_state.castling[Color::WHITE as usize][Castling::QUEENSIDE as usize] = false
                }
                7 => new_state.castling[Color::WHITE as usize][Castling::KINGSIDE as usize] = false,
                54 => {
                    new_state.castling[Color::BLACK as usize][Castling::QUEENSIDE as usize] = false
                }
                63 => {
                    new_state.castling[Color::BLACK as usize][Castling::KINGSIDE as usize] = false
                }
                _ => (),
            }
        }

        /* Update EP target on pawn jumps */
        if self.b.piece_occ(Type::PAWN) & m.src().mask() != 0u64 {
            if (m.dst().rank() as i32 - m.src().rank() as i32).abs() > 1 {
                new_state.ep_target = Some(Square::at(
                    (m.src().rank() + m.dst().rank()) / 2,
                    m.src().file(),
                ));
            }
        }

        /* All moves remove the src piece */
        let mut pc = self.b.remove(m.src());

        /* Promoting moves replace the piece */
        if m.ptype().is_some() {
            pc = Piece::from(m.ptype().unwrap(), self.ctm);
        }

        self.b.place(m.dst(), pc);

        let mut is_legal = true;

        /* Move made, test if king in check */
        bitboard::for_each(
            (self.b.piece_occ(Type::KING) & self.b.color_occ(self.ctm)) | castle_att_mask,
            |s| {
                if self.b.get_ad()[self.ctm.flip() as usize][s.index()] > 0 {
                    is_legal = false;
                }
            },
        );

        /* We update the ply and ctm even if the move is illegal, but change the RV */
        self.ply.push(new_state);
        self.ctm = self.ctm.flip();

        is_legal
    }

    pub fn unmake_move(&mut self, m: Move) {
        //println!("unmaking move {}, current ply = {}", m.to_uci(), self.ply.len());
        let last_state = self.ply.pop().unwrap();

        assert_eq!(last_state.last_move.unwrap(), m);

        /* If move was castle, replace rook */
        if self.b.piece_occ(Type::KING) & m.dst().mask() != 0u64 {
            if (m.src().file() as i32 - m.dst().file() as i32).abs() > 1 {
                let castle_side = match m.dst().file() {
                    2 => Castling::QUEENSIDE,
                    6 => Castling::KINGSIDE,
                    a => panic!("Invalid castling file {}", a),
                };

                /* Place the rook on the old square */
                match castle_side {
                    Castling::QUEENSIDE => {
                        let p = self.b.remove(Square::at(m.src().rank(), 3));
                        self.b.place(Square::at(m.src().rank(), 0), p);
                    }
                    Castling::KINGSIDE => {
                        let p = self.b.remove(Square::at(m.src().rank(), 5));
                        self.b.place(Square::at(m.src().rank(), 7), p);
                    }
                }
            }
        }

        /* Undo move */
        let mut p = self.b.remove(m.dst());

        /* If move was promotion, replace with pawn */
        if m.ptype().is_some() {
            p = Piece::from(Type::PAWN, self.ctm.flip());
        }

        self.b.place(m.src(), p);

        /* If last move was capture, replace piece */
        match last_state.capture_square {
            Some(sq) => {
                self.b.place(sq, last_state.captured.unwrap());
            }
            None => {}
        }

        self.ctm = self.ctm.flip();
    }

    pub fn gen_pseudolegal_moves(&self) -> Vec<Move> {
        let mut output: Vec<Move> = Vec::new();

        /* Gen for pawns with potential to promote */
        let promoting_pawns = self.b.piece_occ(Type::PAWN)
            & self.b.color_occ(self.ctm)
            & movegen::PAWN_PROMOTE_MASK[self.ctm as usize];

        /* Potential move masks */
        let promoting_advances =
            bitboard::shift(promoting_pawns, movegen::PAWN_DIRECTION[self.ctm as usize])
                & !self.b.global_occ();
        let promoting_left_captures = bitboard::shift(
            promoting_pawns & !bitboard::FILES[0],
            movegen::PAWN_LC_DIRECTION[self.ctm as usize],
        ) & self.b.color_occ(self.ctm.flip());
        let promoting_right_captures = bitboard::shift(
            promoting_pawns & !bitboard::FILES[7],
            movegen::PAWN_RC_DIRECTION[self.ctm as usize],
        ) & self.b.color_occ(self.ctm.flip());

        /* Perform movegen */
        bitboard::for_each(promoting_advances, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, Some(Type::QUEEN)));
            output.push(Move::new(src, t, Some(Type::KNIGHT)));
            output.push(Move::new(src, t, Some(Type::ROOK)));
            output.push(Move::new(src, t, Some(Type::BISHOP)));
        });

        bitboard::for_each(promoting_left_captures, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_LC_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, Some(Type::QUEEN)));
            output.push(Move::new(src, t, Some(Type::KNIGHT)));
            output.push(Move::new(src, t, Some(Type::ROOK)));
            output.push(Move::new(src, t, Some(Type::BISHOP)));
        });

        bitboard::for_each(promoting_right_captures, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_RC_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, Some(Type::QUEEN)));
            output.push(Move::new(src, t, Some(Type::KNIGHT)));
            output.push(Move::new(src, t, Some(Type::ROOK)));
            output.push(Move::new(src, t, Some(Type::BISHOP)));
        });

        /* Gen for nonpromoting pawns */
        let np_pawns = self.b.piece_occ(Type::PAWN)
            & self.b.color_occ(self.ctm)
            & !movegen::PAWN_PROMOTE_MASK[self.ctm as usize];

        let ep_mask = match self.ply.last().unwrap().ep_target {
            Some(s) => s.mask(),
            None => 0u64,
        };

        let np_pawn_advances =
            bitboard::shift(np_pawns, movegen::PAWN_DIRECTION[self.ctm as usize])
                & !self.b.global_occ();
        let np_pawn_left_captures = bitboard::shift(
            np_pawns & !bitboard::FILES[0],
            movegen::PAWN_LC_DIRECTION[self.ctm as usize],
        ) & (self.b.color_occ(self.ctm.flip()) | ep_mask);
        let np_pawn_right_captures = bitboard::shift(
            np_pawns & !bitboard::FILES[7],
            movegen::PAWN_RC_DIRECTION[self.ctm as usize],
        ) & (self.b.color_occ(self.ctm.flip()) | ep_mask);

        /* Find jumps by advancing twice */
        let np_pawn_jumps = bitboard::shift(
            np_pawns & movegen::PAWN_JUMP_MASK[self.ctm as usize],
            movegen::PAWN_DIRECTION[self.ctm as usize],
        ) & !self.b.global_occ();
        let np_pawn_jumps =
            bitboard::shift(np_pawn_jumps, movegen::PAWN_DIRECTION[self.ctm as usize])
                & !self.b.global_occ();

        /* Generate moves */
        bitboard::for_each(np_pawn_advances, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, None));
        });

        bitboard::for_each(np_pawn_left_captures, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_LC_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, None));
        });

        bitboard::for_each(np_pawn_right_captures, |t| {
            let src = Square::from_index(
                (t.index() as i32 - movegen::PAWN_RC_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, None));
        });

        bitboard::for_each(np_pawn_jumps, |t| {
            let src = Square::from_index(
                (t.index() as i32 - 2 * movegen::PAWN_DIRECTION[self.ctm as usize]) as usize,
            )
            .unwrap();
            output.push(Move::new(src, t, None));
        });

        /* Generate queen moves */
        bitboard::for_each(
            self.b.piece_occ(Type::QUEEN) & self.b.color_occ(self.ctm),
            |s| {
                let att = attacks::queen(s, self.b.global_occ()) & !self.b.color_occ(self.ctm);

                bitboard::for_each(att, |t| {
                    output.push(Move::new(s, t, None));
                });
            },
        );

        /* Generate rook moves */
        bitboard::for_each(
            self.b.piece_occ(Type::ROOK) & self.b.color_occ(self.ctm),
            |s| {
                let att = attacks::rook(s, self.b.global_occ()) & !self.b.color_occ(self.ctm);

                bitboard::for_each(att, |t| {
                    output.push(Move::new(s, t, None));
                });
            },
        );

        /* Generate bishop moves */
        bitboard::for_each(
            self.b.piece_occ(Type::BISHOP) & self.b.color_occ(self.ctm),
            |s| {
                let att = attacks::bishop(s, self.b.global_occ()) & !self.b.color_occ(self.ctm);

                bitboard::for_each(att, |t| {
                    output.push(Move::new(s, t, None));
                });
            },
        );

        /* Generate knight moves */
        bitboard::for_each(
            self.b.piece_occ(Type::KNIGHT) & self.b.color_occ(self.ctm),
            |s| {
                let att = attacks::knight(s) & !self.b.color_occ(self.ctm);

                bitboard::for_each(att, |t| {
                    output.push(Move::new(s, t, None));
                });
            },
        );

        /* Generate king moves */
        bitboard::for_each(
            self.b.piece_occ(Type::KING) & self.b.color_occ(self.ctm),
            |s| {
                let att = attacks::king(s) & !self.b.color_occ(self.ctm);

                bitboard::for_each(att, |t| {
                    output.push(Move::new(s, t, None));
                });
            },
        );

        /* Generate castling moves */
        let top = self.ply.last().unwrap();

        if top.castling[self.ctm as usize][Castling::QUEENSIDE as usize] {
            if self.b.global_occ()
                & movegen::CASTLE_OCC_MASKS[self.ctm as usize][Castling::QUEENSIDE as usize]
                == 0u64
            {
                let rank = match self.ctm {
                    Color::WHITE => 0,
                    Color::BLACK => 7,
                };

                output.push(Move::new(Square::at(rank, 4), Square::at(rank, 2), None));
            }
        }

        if top.castling[self.ctm as usize][Castling::KINGSIDE as usize] {
            if self.b.global_occ()
                & movegen::CASTLE_OCC_MASKS[self.ctm as usize][Castling::KINGSIDE as usize]
                == 0u64
            {
                let rank = match self.ctm {
                    Color::WHITE => 0,
                    Color::BLACK => 7,
                };

                output.push(Move::new(Square::at(rank, 4), Square::at(rank, 6), None));
            }
        }

        output
    }

    pub fn perft(&mut self, d: usize, p: usize) -> usize {
        if d == 0 {
            return 1;
        }

        let mut total: usize = 0;
        let moves = self.gen_pseudolegal_moves();

        let pr = match self.ply.last().unwrap().last_move {
            Some(m) => m.to_uci() == "b2b3".to_string(),
            None => false,
        };

        for m in moves {
            if p == 0 {
                println!("making 0-ply move {}", m.to_uci());
            }

            if self.make_move(m.clone()) {
                let rs = self.perft(d - 1, p + 1);
                if pr {
                    println!("{}: {}", m.to_uci(), rs);
                }
                total += rs;
            }

            self.unmake_move(m);
        }

        if pr {
            println!("total: {}", total);
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_parse_fen_works() {
        Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
            .unwrap();
    }

    #[test]
    fn position_write_fen_works() {
        assert_eq!(
            Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
                .unwrap()
                .to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn position_invalid_fen_works() {
        /* too many parts */
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 100".to_string()
        )
        .is_none());

        /* bad rank count */
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_none());

        /* bad rank content */
        assert!(Position::new(
            "rnbqkbnrQ/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_none());

        /* bad piece chars */
        assert!(Position::new(
            "rnbqkbnrv/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
        )
        .is_none());

        /* invalid color */
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR k KQkq - 0 1".to_string()
        )
        .is_none());
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR sdfkjns KQkq - 0 1 100".to_string()
        )
        .is_none());

        /* bad hm clock */
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - abc 1".to_string()
        )
        .is_none());

        /* bad fm number */
        assert!(Position::new(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 abc".to_string()
        )
        .is_none());
    }

    #[test]
    fn position_pseudolegal_gen_standard_count() {
        assert_eq!(
            Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
                .unwrap()
                .gen_pseudolegal_moves()
                .len(),
            20
        );
    }

    #[test]
    fn position_pseudolegal_ep_gen_works() {
        let p = Position::new(
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3".to_string(),
        )
        .unwrap();
        let m = Move::from_uci(&"e5d6".to_string()).unwrap();

        assert!(p.gen_pseudolegal_moves().contains(&m));
    }

    #[test]
    fn position_ep_is_legal() {
        let mut p = Position::new(
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3".to_string(),
        )
        .unwrap();
        let m = Move::from_uci(&"e5d6".to_string()).unwrap();

        assert!(p.make_move(m));
    }

    #[test]
    fn position_perft1_nodes() {
        let mut p: Position =
            Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
                .unwrap();

        assert_eq!(p.perft(0, 0), 1);
        assert_eq!(p.perft(1, 0), 20);
        assert_eq!(p.perft(2, 0), 400);
        assert_eq!(p.perft(3, 0), 8902);
        assert_eq!(p.perft(4, 0), 197281);
    }

    #[test]
    fn position_perft2_nodes() {
        let mut p: Position = Position::new(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        )
        .unwrap();

        //assert_eq!(p.perft(1, 0), 48);
        assert_eq!(p.perft(2, 0), 2039);
        assert_eq!(p.perft(3, 0), 97862);
    }

    #[test]
    fn position_perft3_nodes() {
        let mut p: Position =
            Position::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string()).unwrap();

        assert_eq!(p.perft(1, 0), 14);
        assert_eq!(p.perft(2, 0), 191);
        assert_eq!(p.perft(3, 0), 2812);
        assert_eq!(p.perft(4, 0), 43238);
    }

    #[test]
    fn position_perft4_nodes() {
        let mut p: Position = Position::new(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        )
        .unwrap();

        assert_eq!(p.perft(1, 0), 6);
        assert_eq!(p.perft(2, 0), 264);
        assert_eq!(p.perft(3, 0), 9467);
    }

    #[test]
    fn position_perft5_nodes() {
        let mut p: Position = Position::new(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBPPP3/q4N2/Pp4PP/R2Q1RK1 b kq - 0 1".to_string(),
        )
        .unwrap();

        assert_eq!(p.perft(1, 0), 43);
    }

    #[test]
    fn position_perft6_nodes() {
        let mut p: Position = Position::new(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/1PN2Q1p/P1PBBPPP/R3K2R b KQkq - 0 1".to_string(),
        )
        .unwrap();

        assert_eq!(p.perft(1, 0), 42);
    }

    #[test]
    fn position_make_unmake_unchanged() {
        let mut p = Position::new(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string(),
        )
        .unwrap();

        let pclone = p.clone();

        p.make_move(Move::from_uci(&"a2b3".to_string()).unwrap());
        for m in p.gen_pseudolegal_moves() {
            p.make_move(m.clone());
            p.unmake_move(m);
        }
        p.unmake_move(Move::from_uci(&"a2b3".to_string()).unwrap());

        assert_eq!(p, pclone);
    }
}
