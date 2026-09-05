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

#[cfg(test)]
mod tests {
    use crate::types::{Bitboard, File};

    #[test]
    fn file_try_offset() {
        assert_eq!(File::A.try_offset(-1), None);
        assert_eq!(File::A.try_offset(0), Some(File::A));
        assert_eq!(File::A.try_offset(1), Some(File::B));

        assert_eq!(File::D.try_offset(-1), Some(File::C));
        assert_eq!(File::D.try_offset(0), Some(File::D));
        assert_eq!(File::D.try_offset(1), Some(File::E));

        assert_eq!(File::H.try_offset(-7), Some(File::A));
        assert_eq!(File::H.try_offset(-1), Some(File::G));
        assert_eq!(File::H.try_offset(0), Some(File::H));
        assert_eq!(File::H.try_offset(1), None);
    }

    #[test]
    fn file_offset() {
        assert_eq!(File::A.offset(0), File::A);
        assert_eq!(File::A.offset(1), File::B);
        assert_eq!(File::A.offset(7), File::H);

        assert_eq!(File::D.offset(-1), File::C);
        assert_eq!(File::D.offset(0), File::D);
        assert_eq!(File::D.offset(1), File::E);

        assert_eq!(File::H.offset(-7), File::A);
        assert_eq!(File::H.offset(-1), File::G);
        assert_eq!(File::H.offset(0), File::H);
    }

    #[test]
    #[should_panic]
    fn file_offset_panics_a() {
        File::A.offset(-1);
    }

    #[test]
    #[should_panic]
    fn file_offset_panics_h() {
        File::H.offset(1);
    }

    #[test]
    fn file_flip() {
        assert_eq!(File::A.flip(), File::H);
        assert_eq!(File::B.flip(), File::G);
        assert_eq!(File::C.flip(), File::F);
        assert_eq!(File::D.flip(), File::E);
        assert_eq!(File::E.flip(), File::D);
        assert_eq!(File::F.flip(), File::C);
        assert_eq!(File::G.flip(), File::B);
        assert_eq!(File::H.flip(), File::A);
    }

    #[test]
    fn file_bitboard_unique() {
        let mut bb = Bitboard::EMPTY;

        for f in File::ALL.map(|f| f.bitboard()) {
            assert!(bb.is_disjoint(f));
            bb |= f;
        }

        assert_eq!(bb, Bitboard::FULL);
    }
}
