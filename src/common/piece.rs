use crate::def_enum;
use enum_map::Enum;
use std::fmt;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
    pub enum Piece : u8 {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PieceParseError;

impl From<Piece> for char {
    #[inline]
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = PieceParseError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_lowercase() {
            'p' => Ok(Piece::Pawn),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'r' => Ok(Piece::Rook),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err(PieceParseError),
        }
    }
}

impl fmt::Display for Piece {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
