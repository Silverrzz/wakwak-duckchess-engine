use crate::def_enum;
use crate::types::bitboard::Bitboard;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0xff << (8 * self as u8))
    }
}
