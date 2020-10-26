mod attacks;
mod bitboard;
mod gmove;
mod piece;
mod square;

extern crate pretty_env_logger;

#[macro_use] extern crate arr_macro;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

use square::*;

fn main() {
    pretty_env_logger::init();

    info!("Starting neocortex.");

    println!("{}", bitboard::to_pretty(attacks::rook(Square::at(0, 0), 0u64)));
    println!("{}", bitboard::to_pretty(attacks::queen(Square::at(0, 0), 128237u64)));
}
