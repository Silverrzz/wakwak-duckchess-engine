use crate::def_enum;
use crate::types::Bitboard;
use enum_map::Enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
    pub enum File : u8 {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H
    }
}

impl File {
    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0x0101010101010101 << (self as u8))
    }
}
