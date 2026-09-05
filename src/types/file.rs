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
    pub fn try_offset(self, dx: isize) -> Option<Self> {
        Self::try_index((self as usize).wrapping_add_signed(dx))
    }

    #[inline]
    pub fn offset(self, dx: isize) -> Self {
        self.try_offset(dx).expect("File::offset(dx) New index out of bounds")
    }

    #[inline]
    pub const fn flip(self) -> Self {
        Self::index(File::H as usize - self as usize)
    }

    #[inline]
    pub const fn bitboard(self) -> Bitboard {
        Bitboard(0x0101010101010101 << (self as u8))
    }
}
