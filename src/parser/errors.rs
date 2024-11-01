use std::fmt;
use crate::parser::tokens::Loc;

pub enum LexError {
    MalformedNumLit(Loc),
    UnexpectedToken(Loc),
    UnknownLexem(Loc),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MalformedNumLit(loc) => write!(f, "[{loc}] неправильный float литерал"),
            Self::UnexpectedToken(loc) => write!(f, "[{loc}] неожиданное появление"),
            Self::UnknownLexem(loc) => write!(f, "[{loc}] неизвестная лексема"),
        }
    }
}