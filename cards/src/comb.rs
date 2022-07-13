use core::fmt;
use std::fmt::Debug;

use crate::value::Value;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum CombType {
  HighestCard,
  Pair,
  TwoPairs,
  ThreeOfAKind,
  Straight,
  Flush,
  FullHouse,
  FourOfAKind,
  StraightFlush
}

impl fmt::Display for CombType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{:?}", self)
    }
}

impl CombType {
  fn to_u8(&self) -> u8 {
    match self {
        CombType::HighestCard => 0,
        CombType::Pair => 1,
        CombType::TwoPairs => 2,
        CombType::ThreeOfAKind => 3,
        CombType::Straight => 4,
        CombType::Flush => 5,
        CombType::FullHouse => 6,
        CombType::FourOfAKind => 7,
        CombType::StraightFlush => 8
    }
  }
}

#[derive(PartialEq, Eq)]
pub struct Comb { combtype: CombType, value: Value }

impl fmt::Display for Comb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} of {}", self.combtype(), self.value())
    }
}

impl fmt::Debug for Comb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Display::fmt(&self, f)
    }
}

impl PartialOrd for Comb {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      match self.combtype.to_u8().partial_cmp(&other.combtype.to_u8()) {
        Some(core::cmp::Ordering::Equal) => self.value.partial_cmp(&other.value),
        ord => ord
      }
    }
}

impl Ord for Comb {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      match self.partial_cmp(other) {
        Some(ord) => ord,
        None => panic!("HELP ME!")
      }
    }
}

impl Comb {
  pub fn new(combtype: CombType, value_or_highest: Value) -> Self {
    Comb { combtype, value: value_or_highest }
  }

  pub fn combtype(&self) -> &CombType {
    &self.combtype
  }

  pub fn value(&self) -> &Value {
    &self.value
  }
}

