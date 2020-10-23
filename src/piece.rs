/* Piece types */

#[derive(PartialEq, Debug, Copy, Clone)]
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
#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    ptype: Type,
    color: Color,
}

impl Piece {
    pub fn from(t: Type, c: Color) -> Self {
        Piece { ptype: t, color: c }
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
    fn color_to_fen_works() {
        assert_eq!(Color::WHITE.to_fen(), 'w');
        assert_eq!(Color::BLACK.to_fen(), 'b');
    }

    #[test]
    fn color_from_fen_works() {
        assert_eq!(Color::from_fen('w').unwrap(), Color::WHITE);
        assert_eq!(Color::from_fen('b').unwrap(), Color::BLACK);
        assert_eq!(Color::from_fen('a'), None);
    }

    #[test]
    fn type_to_fen_works() {
        assert_eq!(Type::PAWN.to_fen(), 'p');
        assert_eq!(Type::BISHOP.to_fen(), 'b');
        assert_eq!(Type::KNIGHT.to_fen(), 'n');
        assert_eq!(Type::ROOK.to_fen(), 'r');
        assert_eq!(Type::QUEEN.to_fen(), 'q');
        assert_eq!(Type::KING.to_fen(), 'k');
    }

    #[test]
    fn type_from_fen_works() {
        assert_eq!(Type::from_fen('p').unwrap(), Type::PAWN);
        assert_eq!(Type::from_fen('b').unwrap(), Type::BISHOP);
        assert_eq!(Type::from_fen('n').unwrap(), Type::KNIGHT);
        assert_eq!(Type::from_fen('r').unwrap(), Type::ROOK);
        assert_eq!(Type::from_fen('q').unwrap(), Type::QUEEN);
        assert_eq!(Type::from_fen('k').unwrap(), Type::KING);

        assert_eq!(Type::from_fen('a'), None);
    }

    #[test]
    fn piece_to_fen_works() {
        assert_eq!(Piece::from(Type::PAWN, Color::WHITE).to_fen(), 'P');
        assert_eq!(Piece::from(Type::KNIGHT, Color::BLACK).to_fen(), 'n');
        assert_eq!(Piece::from(Type::QUEEN, Color::WHITE).to_fen(), 'Q');
        assert_eq!(Piece::from(Type::KING, Color::BLACK).to_fen(), 'k');
    }

    #[test]
    fn piece_from_fen_works() {
        assert_eq!(
            Piece::from(Type::PAWN, Color::BLACK),
            Piece::from_fen('p').unwrap()
        );
        assert_eq!(
            Piece::from(Type::KNIGHT, Color::WHITE),
            Piece::from_fen('N').unwrap()
        );
        assert_eq!(
            Piece::from(Type::QUEEN, Color::BLACK),
            Piece::from_fen('q').unwrap()
        );
        assert_eq!(
            Piece::from(Type::KING, Color::WHITE),
            Piece::from_fen('K').unwrap()
        );

        assert_eq!(Piece::from_fen('a'), None);
    }
}
