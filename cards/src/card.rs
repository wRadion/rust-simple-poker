use core::fmt;

use crate::value::Value;
use crate::suit::Suit;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Card(pub Value, pub Suit);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => self.1.partial_cmp(&other.1),
            ord => ord,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => panic!("WTF DUDE IDK")
        }
    }
}

impl Card {
    pub fn value(&self) -> &Value {
        &self.0
    }

    pub fn suit(&self) -> &Suit {
        &self.1
    }
}
