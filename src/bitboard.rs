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