/* Piece types */

#[derive(PartialEq, Debug)]
pub enum Type {
    PAWN = 0,
    BISHOP = 1,
    KNIGHT = 2,
    ROOK = 3,
    QUEEN = 4,
    KING = 5,
}

impl Type {
    pub fn to_fen(&self) -> char {
        match self {
            Type::PAWN => 'p',
            Type::BISHOP => 'b',
            Type::KNIGHT => 'n',
            Type::ROOK => 'r',
            Type::QUEEN => 'q',
            Type::KING => 'k',
        }
    }

    pub fn from_fen(c: char) -> Option<Type> {
        match c {
            'p' => Some(Type::PAWN),
            'b' => Some(Type::BISHOP),
            'n' => Some(Type::KNIGHT),
            'r' => Some(Type::ROOK),
            'q' => Some(Type::QUEEN),
            'k' => Some(Type::KING),
            _ => None,
        }
    }
}

/* Piece colors */
#[derive(PartialEq, Debug)]
pub enum Color {
    WHITE = 0,
    BLACK = 1,
}

impl Color {
    pub fn to_fen(&self) -> char {
        match self {
            Color::WHITE => 'w',
            Color::BLACK => 'b',
        }
    }

    pub fn from_fen(c: char) -> Option<Color> {
        match c {
            'w' => Some(Color::WHITE),
            'b' => Some(Color::BLACK),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Piece {
    ptype: Type,
    color: Color,
}

impl Piece {
    pub fn from(t: Type, c: Color) -> Self {
        Piece {
            ptype: t,
            color: c,
        }
    }

    pub fn to_fen(&self) -> char {
        let tc = self.ptype.to_fen();

        if self.color == Color::WHITE {
            return tc.to_uppercase().collect::<Vec<char>>()[0];
        } else {
            return tc;
        }
    }

    pub fn from_fen(fen: char) -> Option<Piece> {
        let ptype = Type::from_fen(fen.to_lowercase().collect::<Vec<char>>()[0]);

        let color = if fen.is_uppercase() {
            Color::WHITE
        } else {
            Color::BLACK
        };
        
        if ptype.is_none() {
            return None;
        }

        Some(Piece {
            ptype: ptype.unwrap(),
            color: color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piece_to_fen_works() {
        assert_eq!(Piece::from(Type::PAWN, Color::WHITE).to_fen(), 'P');
        assert_eq!(Piece::from(Type::KNIGHT, Color::BLACK).to_fen(), 'n');
        assert_eq!(Piece::from(Type::QUEEN, Color::WHITE).to_fen(), 'Q');
        assert_eq!(Piece::from(Type::KING, Color::BLACK).to_fen(), 'k');
    }

    #[test]
    fn piece_from_fen_works() {
        assert_eq!(Piece::from(Type::PAWN, Color::BLACK), Piece::from_fen('p').unwrap());
        assert_eq!(Piece::from(Type::KNIGHT, Color::WHITE), Piece::from_fen('N').unwrap());
        assert_eq!(Piece::from(Type::QUEEN, Color::BLACK), Piece::from_fen('q').unwrap());
        assert_eq!(Piece::from(Type::KING, Color::WHITE), Piece::from_fen('K').unwrap());
    }
}