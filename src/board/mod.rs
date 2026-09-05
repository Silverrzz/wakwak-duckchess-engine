use crate::types::{Bitboard, Color, Piece, Square};
use enum_map::EnumMap;

#[derive(Clone, Copy)]
pub struct Board {
    pieces: EnumMap<Piece, Bitboard>,
    colors: EnumMap<Color, Bitboard>,
    mailbox: EnumMap<Square, Option<Piece>>,
    duck: Option<Square>,
    stm: Color,
}

impl Board {
    #[inline]
    pub fn occupied(&self) -> Bitboard {
        self.colors[Color::White] | self.colors[Color::Black]
    }

    #[inline]
    pub fn colors(&self, color: Color) -> Bitboard {
        self.colors[color]
    }

    #[inline]
    pub fn pieces(&self, piece: Piece) -> Bitboard {
        self.pieces[piece]
    }

    #[inline]
    pub fn colored_pieces(&self, piece: Piece, color: Color) -> Bitboard {
        self.pieces[piece] & self.colors[color]
    }

    #[inline]
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        self.mailbox[sq]
    }

    #[inline]
    pub fn duck(&self) -> Option<Square> {
        self.duck
    }

    #[inline]
    pub fn stm(&self) -> Color {
        self.stm
    }
}
