use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::hand::Hand;
use crate::suit::Suit;
use crate::value::Value;
use crate::card::Card;

pub struct Deck(Vec<Card>);

pub enum DeckError {
  Empty
}

impl Deck {
  pub fn new() -> Deck {
    let mut cards: Vec<Card> = Vec::new();
    for suit in Suit::iter() {
      for value in Value::iter() {
        cards.push(Card(value, suit));
      }
    }
    Deck(cards)
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn deal(&mut self, hand: &mut Hand) -> Option<DeckError> {
    match self.0.pop() {
      Some(card) => { hand.add(card); },
      None => return Some(DeckError::Empty)
    }
    None
  }

  pub fn shuffle(&mut self) {
    let mut rng = rand::thread_rng();
    self.0.shuffle(&mut rng);
  }
}
