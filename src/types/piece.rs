use crate::def_enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Piece : u8 {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King
    }
}
