use crate::common::{Bitboard, Color, File};
use crate::def_enum;
use core::fmt;
use enum_map::Enum;
use std::fmt::Formatter;

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
    pub const fn try_offset(self, dx: isize) -> Option<Self> {
        Self::try_index((self as usize).wrapping_add_signed(dx))
    }

    #[inline]
    pub const fn offset(self, dx: isize) -> Self {
        self.try_offset(dx)
            .expect("Rank::offset(dx) New index out of bounds")
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RankParseError;

impl From<Rank> for char {
    #[inline]
    fn from(rank: Rank) -> Self {
        char::from(b'1' + rank as u8)
    }
}

impl TryFrom<char> for Rank {
    type Error = RankParseError;

    #[inline]
    fn try_from(c: char) -> Result<Self, Self::Error> {
        c.to_digit(10)
            .and_then(|i| Rank::try_index(i as usize))
            .ok_or(RankParseError)
    }
}

impl fmt::Display for Rank {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Bitboard, Rank};

    #[test]
    fn rank_try_offset() {
        assert_eq!(Rank::First.try_offset(-1), None);
        assert_eq!(Rank::First.try_offset(0), Some(Rank::First));
        assert_eq!(Rank::First.try_offset(1), Some(Rank::Second));

        assert_eq!(Rank::Fourth.try_offset(-1), Some(Rank::Third));
        assert_eq!(Rank::Fourth.try_offset(0), Some(Rank::Fourth));
        assert_eq!(Rank::Fourth.try_offset(1), Some(Rank::Fifth));

        assert_eq!(Rank::Eighth.try_offset(-7), Some(Rank::First));
        assert_eq!(Rank::Eighth.try_offset(-1), Some(Rank::Seventh));
        assert_eq!(Rank::Eighth.try_offset(0), Some(Rank::Eighth));
        assert_eq!(Rank::Eighth.try_offset(1), None);
    }

    #[test]
    fn rank_offset() {
        assert_eq!(Rank::First.offset(0), Rank::First);
        assert_eq!(Rank::First.offset(1), Rank::Second);
        assert_eq!(Rank::First.offset(7), Rank::Eighth);

        assert_eq!(Rank::Fourth.offset(-1), Rank::Third);
        assert_eq!(Rank::Fourth.offset(0), Rank::Fourth);
        assert_eq!(Rank::Fourth.offset(1), Rank::Fifth);

        assert_eq!(Rank::Eighth.offset(-7), Rank::First);
        assert_eq!(Rank::Eighth.offset(-1), Rank::Seventh);
        assert_eq!(Rank::Eighth.offset(0), Rank::Eighth);
    }

    #[test]
    #[should_panic]
    fn rank_offset_panics_first() {
        Rank::First.offset(-1);
    }

    #[test]
    #[should_panic]
    fn rank_offset_panics_eighth() {
        Rank::Eighth.offset(1);
    }

    #[test]
    fn rank_flip() {
        assert_eq!(Rank::First.flip(), Rank::Eighth);
        assert_eq!(Rank::Second.flip(), Rank::Seventh);
        assert_eq!(Rank::Third.flip(), Rank::Sixth);
        assert_eq!(Rank::Fourth.flip(), Rank::Fifth);
        assert_eq!(Rank::Fifth.flip(), Rank::Fourth);
        assert_eq!(Rank::Sixth.flip(), Rank::Third);
        assert_eq!(Rank::Seventh.flip(), Rank::Second);
        assert_eq!(Rank::Eighth.flip(), Rank::First);
    }

    #[test]
    fn rank_bitboard_unique() {
        let mut bb = Bitboard::EMPTY;

        for r in Rank::ALL.map(|r| r.bitboard()) {
            assert!(bb.is_disjoint(r));
            bb |= r;
        }

        assert_eq!(bb, Bitboard::FULL);
    }
}
