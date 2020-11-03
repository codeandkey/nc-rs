use crate::square::*;

pub const RANKS: [u64; 8] = [
    0x00000000000000FF,
    0x00000000000000FF << 8,
    0x00000000000000FF << 16,
    0x00000000000000FF << 24,
    0x00000000000000FF << 32,
    0x00000000000000FF << 40,
    0x00000000000000FF << 48,
    0x00000000000000FF << 56,
];

pub const FILES: [u64; 8] = [
    0x0101010101010101,
    0x0101010101010101 << 1,
    0x0101010101010101 << 2,
    0x0101010101010101 << 3,
    0x0101010101010101 << 4,
    0x0101010101010101 << 5,
    0x0101010101010101 << 6,
    0x0101010101010101 << 7,
];

pub fn to_pretty(b: u64) -> String {
    let mut output: String = String::new();

    for r in (0..8).rev() {
        for f in 0..8 {
            if b & (1u64 << (r * 8 + f)) != 0u64 {
                output.push('#');
            } else {
                output.push('.');
            }
        }

        if r > 0 {
            output.push('\n');
        }
    }

    output
}

pub fn for_each<F>(mut b: u64, mut c: F)
where
    F: FnMut(Square),
{
    while b != 0u64 {
        let s: Square = Square::from_index(b.trailing_zeros() as usize).unwrap();
        b ^= s.mask();
        c(s);
    }
}

pub fn shift(b: u64, d: i32) -> u64 {
    if (d) > 0 {
        b.overflowing_shl(d as u32).0
    } else {
        b.overflowing_shr((-d) as u32).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitboard_to_pretty_works() {
        assert_eq!(
            to_pretty(0xfe79e8a5c9b8095c),
            ".#######\n#..####.\n...#.###\n#.#..#.#\n#..#..##\n...###.#\n#..#....\n..###.#."
        );
    }
}
