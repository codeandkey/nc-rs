use crate::piece::*;
use crate::square::*;

#[derive(PartialEq, Debug)]
pub struct Move {
    m_src: Square,
    m_dst: Square,
    m_ptype: Option<Type>,
}

impl Move {
    pub fn new(src: Square, dst: Square, ptype: Option<Type>) -> Move {
        Move {
            m_src: src,
            m_dst: dst,
            m_ptype: ptype,
        }
    }

    pub fn to_uci(&self) -> String {
        let mut out: String = self.m_src.to_str();
        out.push_str(&self.m_dst.to_str());

        if self.m_ptype.is_some() {
            out.push(self.m_ptype.unwrap().to_fen());
        }

        out
    }

    pub fn to_uci_withnull(inp: &Option<Move>) -> String {
        match inp {
            Some(m) => m.to_uci(),
            None => "0000".to_string(),
        }
    }

    pub fn from_uci(inp: &String) -> Option<Move> {
        if inp.len() != 4 && inp.len() != 5 {
            return None;
        }

        if inp == "0000" {
            return None;
        }

        let src: Option<Square> = Square::from_uci(&inp[0..2].to_string());
        let dst: Option<Square> = Square::from_uci(&inp[2..4].to_string());

        if src.is_none() || dst.is_none() {
            return None;
        }

        let mut ptype: Option<Type> = None;

        if inp.len() == 5 {
            ptype = Type::from_fen(inp.as_bytes()[4] as char);

            if ptype.is_none() {
                return None;
            }
        }

        Some(Move {
            m_src: src.unwrap(),
            m_dst: dst.unwrap(),
            m_ptype: ptype,
        })
    }

    pub fn src(&self) -> Square {
        self.m_src
    }

    pub fn dst(&self) -> Square {
        self.m_dst
    }

    pub fn ptype(&self) -> Option<Type> {
        self.m_ptype.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_new_works() {
        let src: Square = Square::from_uci(&"a1".to_string()).unwrap();
        let dst: Square = Square::from_uci(&"e2".to_string()).unwrap();
        let ptype: Option<Type> = Some(Type::QUEEN);

        let m: Move = Move::new(src, dst, ptype);

        assert_eq!(m.src(), src);
        assert_eq!(m.dst(), dst);
        assert_eq!(m.ptype(), ptype);
    }

    #[test]
    fn move_to_uci_works() {
        assert_eq!(Move::new(Square::at(0, 0), Square::at(4, 5), None).to_uci(), "a1f5");

        assert_eq!(Move::new(Square::at(6, 1), Square::at(7, 1), Some(Type::QUEEN)).to_uci(), "b7b8q");
        assert_eq!(Move::new(Square::at(6, 1), Square::at(7, 1), Some(Type::BISHOP)).to_uci(), "b7b8b");
        assert_eq!(Move::new(Square::at(6, 1), Square::at(7, 1), Some(Type::KNIGHT)).to_uci(), "b7b8n");
        assert_eq!(Move::new(Square::at(6, 1), Square::at(7, 1), Some(Type::ROOK)).to_uci(), "b7b8r");
    }

    #[test]
    fn move_to_uci_withnull_works() {
        assert_eq!(Move::to_uci_withnull(&None), "0000");
        assert_eq!(Move::to_uci_withnull(&Some(Move::new(Square::at(6, 1), Square::at(7, 1), Some(Type::KNIGHT)))), "b7b8n");
    }

    #[test]
    fn move_from_uci_works() {
        assert_eq!(Move::from_uci(&"0000".to_string()), None);
        assert_eq!(Move::from_uci(&"a1e4".to_string()).unwrap(), Move::new(Square::at(0, 0), Square::at(3, 4), None));
        assert_eq!(Move::from_uci(&"c7c8q".to_string()).unwrap(), Move::new(Square::at(6, 2), Square::at(7, 2), Some(Type::QUEEN)));
    }
}