use crate::board::*;
use crate::gmove::*;
use crate::piece::*;
use crate::square::*;
use crate::zobrist::*;

pub enum Castling {
    QUEENSIDE = 0,
    KINGSIDE = 1,
}

pub struct State {
    ep_target: Option<Square>, /* En-passant target square */
    captured: Option<Piece>, /* Piece captured */
    ad: [[u32; 64]; 2], /* A/D maps */
    fm_number: u32, /* Move number */
    hm_clock: u32, /* Halfmove clock */
    castling: [[bool; 2]; 2], /* Castling rights */
    last_move: Option<Move>, /* Last move */
}

pub struct Position {
    ply: Vec<State>,
    b: Board,
    ctm: Color,
}

impl Position {
    pub fn new(fen: String) -> Option<Position> {
        let parts: Vec<&str> = fen.split(' ').collect();

        if parts.len() != 6 {
            error!("Invalid number of FEN parts: expected 6, read {}", parts.len());
            return None;
        }

        /* Parse board data */
        let ranks: Vec<&str> = parts[0].split('/').collect();

        if ranks.len() != 8 {
            error!("Invalid number of FEN ranks: expected 8, read {}", ranks.len());
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
                        Some(p) => b.place(Square::at(r as usize, f as usize), p),
                        None => {
                            error!("Invalid piece character '{}' in FEN", c);
                            return None;
                        },
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
            ply: vec!(State {
                ep_target: ep_target,
                captured: None,
                ad: [[0; 64]; 2],
                fm_number: fm_number,
                hm_clock: hm_clock,
                castling: rights,
                last_move: None,
            }),
            b: b,
            ctm: ctm,
        })
    }

    pub fn to_fen() -> String {
        String::new()
    }
}
