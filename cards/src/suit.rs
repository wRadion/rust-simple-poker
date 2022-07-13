use core::fmt;
use strum_macros::EnumIter;

#[allow(dead_code)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Club,
    Diamond
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Diamond => "♢",
            Suit::Heart => "♡",
            Suit::Spade => "♠",
            Suit::Club => "♣"
        };

        write!(f, "{}", suit)
    }
}

impl fmt::Debug for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
