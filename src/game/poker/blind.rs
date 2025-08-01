use std::fmt;
use std::fmt::Formatter;
use crate::deck::{Card, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blind {
    NONE = 0,
    SMALL = 1,
    BIG = 2
}

impl fmt::Display for Blind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let res = match self {
            Blind::NONE => "",
            Blind::SMALL => "SMALL BLIND",
            Blind::BIG => "BIG BLIND",
            _ => "unknown blind"
        };

        write!(f, "{}", res)
    }
}
