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

#[cfg(test)]
mod tests {
    use crate::types::{MoveFlag, Piece};

    #[test]
    fn move_flags() {
        let flags = [
            MoveFlag::Normal,
            MoveFlag::DoublePush,
            MoveFlag::LongCastling,
            MoveFlag::ShortCastling,
            MoveFlag::PromotionQueen,
            MoveFlag::PromotionRook,
            MoveFlag::PromotionBishop,
            MoveFlag::PromotionKnight,
            MoveFlag::Capture,
            MoveFlag::EnPassant,
            MoveFlag::CapturePromotionQueen,
            MoveFlag::CapturePromotionRook,
            MoveFlag::CapturePromotionBishop,
            MoveFlag::CapturePromotionKnight,
        ];

        let expected_promo = [
            None,
            None,
            None,
            None,
            Some(Piece::Queen),
            Some(Piece::Rook),
            Some(Piece::Bishop),
            Some(Piece::Knight),
            None,
            None,
            Some(Piece::Queen),
            Some(Piece::Rook),
            Some(Piece::Bishop),
            Some(Piece::Knight),
        ];

        for (&flag, &expected_promo) in flags.iter().zip(expected_promo.iter()) {
            assert_eq!(flag.promotion(), expected_promo);
            assert_eq!(flag.is_promotion(), expected_promo.is_some());
        }

        let expected_capture = [
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            true,
            true,
            true,
            true,
            true,
            true,
        ];

        for (&flag, &expected_capture) in flags.iter().zip(expected_capture.iter()) {
            assert_eq!(flag.is_capture(), expected_capture);
        }
    }
}