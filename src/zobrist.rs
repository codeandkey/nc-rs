use rand::Rng;

fn gen_ep_keys() -> [u64; 8] {
    let mut rng = rand::thread_rng();
    arr![rng.gen::<u64>(); 8]
}

fn gen_castle_keys() -> [[u64; 2]; 2] {
    let mut rng = rand::thread_rng();
    [arr![rng.gen::<u64>(); 2], arr![rng.gen::<u64>(); 2]]
}

fn gen_piece_keys() -> [[u64; 12]; 64] {
    let mut rng = rand::thread_rng();
    let mut out: [[u64; 12]; 64] = [[0; 12]; 64];

    for s in 0..64 {
        out[s] = arr![rng.gen::<u64>(); 12];
    }

    out
}

lazy_static! {
    pub static ref BLACK_TO_MOVE: u64 = rand::thread_rng().gen::<u64>();
    pub static ref EN_PASSANT: [u64; 8] = gen_ep_keys();
    pub static ref CASTLE: [[u64; 2]; 2] = gen_castle_keys();
    pub static ref PIECE: [[u64; 12]; 64] = gen_piece_keys();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zobrist_keys_are_unique() {
        let mut keys: Vec<u64> = Vec::new();

        keys.push(*BLACK_TO_MOVE);
        keys.append(&mut Vec::from(*EN_PASSANT));

        for i in 0..2 {
            keys.append(&mut Vec::from(CASTLE[i]));
        }

        for i in 0..12 {
            keys.append(&mut Vec::from(PIECE[i]));
        }

        /* Test for uniqueness */
        keys.sort();

        let sz = keys.len();
        keys.dedup();

        assert!(sz == keys.len());
    }
}
