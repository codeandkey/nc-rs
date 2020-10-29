use crate::square::*;

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
