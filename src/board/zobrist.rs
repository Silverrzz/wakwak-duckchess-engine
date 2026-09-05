use crate::common::{Color, File, Piece, Square};

#[derive(Debug, Clone)]
pub struct SplitMix64 {
    pub state: u64,
}

impl SplitMix64 {
    #[inline]
    pub const fn new(state: u64) -> SplitMix64 {
        SplitMix64 { state }
    }

    #[inline]
    pub const fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15u64);

        let mut temp = self.state;
        temp = (temp ^ (temp >> 30)).wrapping_mul(0xbf58476d1ce4e5b9u64);
        temp = (temp ^ (temp >> 27)).wrapping_mul(0x94d049bb133111ebu64);

        temp ^ (temp >> 31)
    }
}

/*----------------------------------------------------------------*/

#[derive(Debug, Copy, Clone)]
pub struct Zobrist {
    pub pieces: [[[u64; Square::COUNT]; Piece::COUNT]; Color::COUNT],
    pub castling_rights: [[u64; File::COUNT]; Color::COUNT],
    pub en_passant: [u64; File::COUNT],
    pub duck: [u64; Square::COUNT],
    pub stm: u64,
}

impl Zobrist {
    #[inline]
    pub const fn new(seed: u64) -> Zobrist {
        let mut rng = SplitMix64::new(seed);
        let mut zobrist = Zobrist {
            pieces: [[[0; Square::COUNT]; Piece::COUNT]; Color::COUNT],
            castling_rights: [[0; File::COUNT]; Color::COUNT],
            en_passant: [0; File::COUNT],
            duck: [0; Square::COUNT],
            stm: 0,
        };

        let mut color = 0;
        while color < Color::COUNT {
            let mut piece = 0;
            while piece < Piece::COUNT {
                let mut sq = 0;
                while sq < Square::COUNT {
                    zobrist.pieces[color][piece][sq] = rng.next();
                    zobrist.duck[sq] = rng.next();
                    sq += 1;
                }

                piece += 1;
            }

            let mut file = 0;
            while file < File::COUNT {
                zobrist.castling_rights[color][file] = rng.next();
                zobrist.en_passant[file] = rng.next();
                file += 1;
            }

            color += 1;
        }

        zobrist.stm = rng.next();
        zobrist
    }

    #[inline]
    pub const fn piece(&self, sq: Square, piece: Piece, color: Color) -> u64 {
        self.pieces[color as usize][piece as usize][sq as usize]
    }

    #[inline]
    pub const fn castling_rights(&self, color: Color, file: File) -> u64 {
        self.castling_rights[color as usize][file as usize]
    }

    #[inline]
    pub const fn en_passant(&self, file: File) -> u64 {
        self.en_passant[file as usize]
    }

    #[inline]
    pub const fn duck(&self, sq: Square) -> u64 {
        self.duck[sq as usize]
    }
}

pub static ZOBRIST: Zobrist = Zobrist::new(0xe65f2056120a3513);
