use crate::types::{Piece, Square};
use std::num::NonZeroU32;

/// A duck chess move. Bit Layout:
/// - Bits 0-5: Source Square
/// - Bits 6-11: Target Square
/// - Bits 12-17: Duck Square
/// - Bits 18-21: Move Flag
/// - Bits 22-31: Unused
pub struct Move(NonZeroU32);

impl Move {
    #[inline]
    pub const fn new(src: Square, dest: Square, duck: Square, flag: MoveFlag) -> Move {
        let mut bits = 0;
        bits |= src as u32;
        bits |= (dest as u32) << 6;
        bits |= (duck as u32) << 12;
        bits |= (flag as u32) << 18;

        Move(NonZeroU32::new(bits).unwrap())
    }

    #[inline]
    pub const fn src(self) -> Square {
        Square::index(self.0.get() as usize & 0b111111)
    }

    #[inline]
    pub const fn dest(self) -> Square {
        Square::index((self.0.get() as usize >> 6) & 0b111111)
    }

    #[inline]
    pub const fn duck(self) -> Square {
        Square::index((self.0.get() as usize >> 12) & 0b111111)
    }

    #[inline]
    pub const fn flag(self) -> MoveFlag {
        // SAFETY: All moves created using `Move::new()` will have a valid `MoveFlag` bit pattern.

        unsafe { core::mem::transmute((self.0.get() >> 18) as u8) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveFlag {
    Normal = 0x0,
    DoublePush = 0x1,
    LongCastling = 0x2,
    ShortCastling = 0x3,
    PromotionQueen = 0x4,
    PromotionRook = 0x5,
    PromotionBishop = 0x6,
    PromotionKnight = 0x7,
    Capture = 0x8,
    EnPassant = 0x9,
    CapturePromotionQueen = 0xC,
    CapturePromotionRook = 0xD,
    CapturePromotionBishop = 0xE,
    CapturePromotionKnight = 0xF,
}

impl MoveFlag {
    #[inline]
    pub const fn promotion(self) -> Option<Piece> {
        if !self.is_promotion() {
            return None;
        }

        const LOOKUP: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

        Some(LOOKUP[self as usize & 0x3])
    }

    #[inline]
    pub const fn is_castling(self) -> bool {
        matches!(self, MoveFlag::LongCastling | MoveFlag::ShortCastling)
    }

    #[inline]
    pub const fn is_promotion(self) -> bool {
        (self as u8 & 0x4) != 0
    }

    #[inline]
    pub const fn is_capture_promotion(self) -> bool {
        (self as u8 & 0xC) != 0
    }

    #[inline]
    pub const fn is_capture(self) -> bool {
        (self as u8 & 0x8) != 0
    }
}
