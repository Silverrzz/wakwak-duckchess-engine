use crate::def_enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Color : u8 {
        White,
        Black
    }
}
