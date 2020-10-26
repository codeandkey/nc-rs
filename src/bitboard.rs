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

pub fn for_each<F>(mut b: u64, mut c: F) where F: FnMut(Square) {
    while b != 0u64 {
        let s: Square = Square::from_index(b.trailing_zeros() as usize).unwrap();
        b ^= s.mask();
        c(s);
    }
}
