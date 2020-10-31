mod attacks;
mod bitboard;
mod board;
mod gmove;
mod piece;
mod position;
mod square;
mod zobrist;

extern crate pretty_env_logger;
extern crate rand;

#[macro_use]
extern crate arr_macro;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use square::*;

fn main() {
    pretty_env_logger::init();

    info!("Starting neocortex.");

    println!(
        "{}",
        bitboard::to_pretty(attacks::rook(Square::at(0, 0), 0u64))
    );
    println!(
        "{}",
        bitboard::to_pretty(attacks::queen(Square::at(0, 0), 128237u64))
    );

    let p = position::Position::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap();
    let m = p.gen_pseudolegal_moves();

    for _m in m {
        println!("Move: {}", _m.to_uci());
    }
}
