use crate::def_enum;
use enum_map::Enum;

def_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Enum)]
    pub enum Color : u8 {
        White,
        Black
    }
}
