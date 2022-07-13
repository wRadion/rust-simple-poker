use core::fmt;

use crate::card::Card;
use crate::comb::{Comb, CombType};

#[derive(Clone)]
pub struct Hand(pub Vec<Card>);

#[derive(Debug)]
pub enum HandError {
  TooManyCards
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let cards: Vec<String> = self.0.iter().map(|c| format!("{}", c).clone()).collect();
      write!(f, "({})", cards.join(", "))
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
      if self.0.len() != other.0.len() {
        return false;
      }

      let mut our_cards = self.0.clone();
      our_cards.sort();

      let mut their_cards = other.0.clone();
      their_cards.sort();

      for i in 0..our_cards.len() {
        if our_cards[i] != their_cards[i] {
          return false;
        }
      }

      true
    }
}

impl Hand {
  pub fn empty() -> Self {
    Hand(vec![])
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn from(cards: Vec<Card>) -> Result<Self, HandError> {
    if cards.len() > 5 {
      Err(HandError::TooManyCards)
    } else {
      Ok(Hand(cards))
    }
  }

  pub fn add(&mut self, card: Card) -> bool {
    let mut my_clone = self.clone();
    my_clone.0.push(card);

    if my_clone.is_valid() {
      self.0.push(card);
      true
    } else {
      false
    }
  }

  pub fn remove(&mut self, index: &usize) {
    if index < &self.0.len() {
      self.0.remove(*index);
    }
  }

  pub fn is_valid(&self) -> bool {
    if self.0.len() > 5 {
      return false;
    }

    for card in &self.0 {
      if self.0.iter().filter(|c| *c == card).count() > 1 {
        return false;
      }
    }

    true
  }

  pub fn find_highest_comb(&self) -> Comb {
    let mut cards = self.0.clone();
    cards.sort_by(|a, b| b.cmp(a));
    let cards = cards;
    let highest = cards[0];

    // Flush
    let flush = cards.iter().all(|c| c.suit() == highest.suit());

    // Straight
    let mut straight = true;
    {
      let mut expect = cards[0].value().to_u8() - 1;
      for i in 1..cards.len() {
        if cards[i].value().to_u8() != expect {
          straight = false;
          break;
        }
        expect -= 1;
      }
    }

    // Straight and/or Flush
    match (straight, flush) {
      (true, true) => return Comb::new(CombType::StraightFlush, *highest.value()),
      (false, true) => return Comb::new(CombType::Flush, *highest.value()),
      (true, false) => return Comb::new(CombType::Straight, *highest.value()),
      _ => ()
    }

    // Pairs, Threes, Fours
    let mut combs: (Option<Comb>, Option<Comb>) = (None, None);
    for card in cards.clone() {
      if let Some(comb) = &combs.0 { if comb.value() == card.value() { continue; } }
      if let Some(comb) = &combs.1 { if comb.value() == card.value() { continue ;} }

      let comb = match cards.iter().filter(|c| c.value() == card.value()).count() {
        4 => return Comb::new(CombType::FourOfAKind, *card.value()),
        3 => Some(Comb::new(CombType::ThreeOfAKind, *card.value())),
        2 => Some(Comb::new(CombType::Pair, *card.value())),
        _ => None
      };

      if let Some(comb) = comb {
        match combs {
          (None, None) => { combs.0 = Some(comb); },
          (Some(_), None) => { combs.1 = Some(comb); },
          _ => panic!("This is not supposed to happen!")
        }
      }
    }

    // FullHouse, TwoPairs
    match combs {
      (Some(comb), None) => comb,
      (Some(pair1), Some(pair2)) if *pair1.combtype() == CombType::Pair && *pair2.combtype() == CombType::Pair =>
        Comb::new(CombType::TwoPairs, *pair1.value().max(pair2.value())),
      (Some(threeofakind), Some(pair)) | (Some(pair), Some(threeofakind)) if *threeofakind.combtype() == CombType::ThreeOfAKind && *pair.combtype() == CombType::Pair =>
        Comb::new(CombType::FullHouse, *threeofakind.value()),
      (None, None) => Comb::new(CombType::HighestCard, *highest.value()),
      _ => panic!("This is not supposed to happen!")
    }
  }
}
