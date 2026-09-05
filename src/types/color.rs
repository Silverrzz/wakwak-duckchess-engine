use crate::def_enum;
use enum_map::Enum;
use std::ops::Not;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
    pub enum Color : u8 {
        White,
        Black
    }
}

impl Not for Color {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
