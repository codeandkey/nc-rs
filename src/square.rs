pub enum Direction {
    EAST = 1,
    WEST = -1,
    NORTH = 8,
    SOUTH = -8,
    NORTHWEST = 7,
    NORTHEAST = 9,
    SOUTHWEST = -9,
    SOUTHEAST = -7,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Square {
    ind: usize,
    r: usize,
    f: usize,
}

impl Square {
    pub fn at(rank: usize, file: usize) -> Square {
        assert!(rank < 8);
        assert!(file < 8);

        Square {
            ind: rank * 8 + file,
            r: rank,
            f: file,
        }
    }

    pub fn to_str(&self) -> String {
        let mut out = String::new();

        out.push((('a' as u8) + self.f as u8) as char);
        out.push((('1' as u8) + self.r as u8) as char);

        out
    }

    pub fn from_uci(inp: &String) -> Option<Square> {
        if inp == "-" || inp.len() != 2 {
            return None;
        }

        let file: i32 = (inp.as_bytes()[0] as i32) - ('a' as i32);
        let rank: i32 = (inp.as_bytes()[1] as i32) - ('1' as i32);

        if file < 0 || file >= 8 || rank < 0 || rank >= 8 {
            return None;
        }

        Some(Square {
            r: rank as usize,
            f: file as usize,
            ind: (rank * 8 + file) as usize,
        })
    }

    pub fn from_index(inp: usize) -> Option<Square> {
        if inp > 64 {
            return None;
        }

        Some(Square {
            r: inp / 8,
            f: inp % 8,
            ind: inp,
        })
    }

    pub fn to_str_withnull(inp: &Option<Square>) -> String {
        match inp {
            Some(s) => s.to_str(),
            None => "-".to_string(),
        }
    }

    pub fn mask(&self) -> u64 {
        (1 as u64) << self.ind
    }

    pub fn rank(&self) -> usize {
        self.r
    }

    pub fn file(&self) -> usize {
        self.f
    }

    pub fn index(&self) -> usize {
        self.ind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_at_works() {
        assert_eq!(Square::at(0, 0).rank(), 0);
        assert_eq!(Square::at(0, 0).file(), 0);
        assert_eq!(Square::at(5, 0).rank(), 5);
        assert_eq!(Square::at(0, 5).file(), 5);
    }

    #[test]
    fn square_to_str_works() {
        assert_eq!(Square::at(0, 0).to_str(), "a1");
        assert_eq!(Square::at(3, 3).to_str(), "d4");
        assert_eq!(Square::at(7, 1).to_str(), "b8");
        assert_eq!(Square::at(5, 4).to_str(), "e6");
    }

    #[test]
    fn square_to_str_withnull_works() {
        assert_eq!(Square::to_str_withnull(&None), "-");
        assert_eq!(Square::to_str_withnull(&Some(Square::at(5, 5))), "f6");
    }

    #[test]
    fn square_from_uci_works() {
        assert_eq!(
            Square::from_uci(&"e1".to_string()).unwrap(),
            Square::at(0, 4)
        );
        assert_eq!(
            Square::from_uci(&"b4".to_string()).unwrap(),
            Square::at(3, 1)
        );
        assert_eq!(
            Square::from_uci(&"g2".to_string()).unwrap(),
            Square::at(1, 6)
        );
        assert_eq!(
            Square::from_uci(&"a8".to_string()).unwrap(),
            Square::at(7, 0)
        );

        assert_eq!(Square::from_uci(&"-".to_string()), None);
        assert_eq!(Square::from_uci(&"I'MTOOLONG!".to_string()), None);
        assert_eq!(Square::from_uci(&"b0".to_string()), None);
        assert_eq!(Square::from_uci(&"b9".to_string()), None);
        assert_eq!(Square::from_uci(&"A1".to_string()), None);
        assert_eq!(Square::from_uci(&"i1".to_string()), None);
    }
}
