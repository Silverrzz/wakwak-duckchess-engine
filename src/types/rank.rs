use crate::def_enum;
use crate::types::{Bitboard, Color};
use enum_map::Enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
    pub enum Rank : u8 {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
        Eighth
    }
}

impl Rank {
    #[inline]
    pub fn try_offset(self, dx: isize) -> Option<Self> {
        Self::try_index((self as usize).wrapping_add_signed(dx))
    }

    #[inline]
    pub fn offset(self, dx: isize) -> Self {
        self.try_offset(dx).expect("Rank::offset(dx) New index out of bounds")
    }
    
    #[inline]
    pub const fn flip(self) -> Self {
        Self::index(Rank::Eighth as usize - self as usize)
    }

    #[inline]
    pub const fn relative_to(self, color: Color) -> Self {
        match color {
            Color::White => self,
            Color::Black => self.flip(),
        }
    }

    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0xff << (8 * self as u8))
    }
}
