pub mod value;
pub mod suit;
pub mod card;
pub mod hand;
pub mod deck;
pub mod comb;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::card::Card;
    use crate::value::Value::*;
    use crate::suit::Suit::*;
    use crate::hand::Hand;
    use crate::deck::Deck;
    use crate::comb::*;

    #[test]
    fn hand_new_test() {
      assert_eq!(Hand::empty(), Hand::from(vec![]).unwrap());
    }

    #[test]
    fn hand_eq_one_card_tests() {
        assert_eq!(Hand(vec![Card(Five, Heart)]), Hand(vec![Card(Five, Heart)]));
        assert_ne!(Hand(vec![Card(Ace, Spade)]), Hand(vec![Card(King, Diamond)]))
    }

    #[test]
    fn hand_eq_many_cards_tests() {
        assert_eq!(
            Hand(vec![Card(Two, Club), Card(Jack, Spade), Card(Ten, Diamond)]),
            Hand(vec![Card(Jack, Spade), Card(Two, Club), Card(Ten, Diamond)])
        );
        assert_eq!(
            Hand(vec![Card(Three, Heart), Card(Ace, Heart)]),
            Hand(vec![Card(Ace, Heart), Card(Three, Heart)])
        );
        assert_ne!(
            Hand(vec![Card(Three, Heart), Card(Ace, Heart)]),
            Hand(vec![Card(Three, Heart), Card(Ace, Club)])
        );
    }

    #[test]
    fn deck_new_test() {
        assert_eq!(Deck::new().len(), 52);
    }

    #[test]
    fn deck_deal_no_arg_test() {
        let mut hand = Hand::empty();
        let mut deck = Deck::new();

        assert_eq!(hand.len(), 0);
        assert_eq!(deck.len(), 52);

        deck.deal(&mut hand);

        assert_eq!(hand.len(), 1);
        assert_eq!(deck.len(), 51);
    }

    #[test]
    fn comb_compare_tests() {
        assert!(Comb::new(CombType::Pair, Two) < Comb::new(CombType::Pair, Three));
        assert!(Comb::new(CombType::Pair, Ace) < Comb::new(CombType::ThreeOfAKind, Two));
        assert!(Comb::new(CombType::Flush, Jack) > Comb::new(CombType::Flush, Ten));
        assert!(Comb::new(CombType::Straight, Six) > Comb::new(CombType::Straight, Five));
        assert!(Comb::new(CombType::StraightFlush, Ace) == Comb::new(CombType::StraightFlush, Ace));
        assert!(Comb::new(CombType::FullHouse, Queen) == Comb::new(CombType::FullHouse, Queen));
    }

    #[test]
    fn hand_find_highest_comb_tests() {
        assert_eq!(
            Hand(vec![Card(Six, Club), Card(King, Heart), Card(Queen, Heart), Card(Four, Spade), Card(Ten, Club)]).find_highest_comb(),
            Comb::new(CombType::HighestCard, King)
        );
        assert_eq!(
            Hand(vec![Card(Two, Spade), Card(Jack, Diamond), Card(Three, Spade), Card(Two, Club), Card(Five, Diamond)]).find_highest_comb(),
            Comb::new(CombType::Pair, Two)
        );
        assert_eq!(
            Hand(vec![Card(Two, Spade), Card(Jack, Diamond), Card(Jack, Spade), Card(Two, Club), Card(Five, Diamond)]).find_highest_comb(),
            Comb::new(CombType::TwoPairs, Jack)
        );
        assert_eq!(
            Hand(vec![Card(Five, Heart), Card(Five, Diamond), Card(Ten, Heart), Card(Ace, Diamond), Card(Five, Club)]).find_highest_comb(),
            Comb::new(CombType::ThreeOfAKind, Five)
        );
        assert_eq!(
            Hand(vec![Card(Two, Spade), Card(Five, Diamond), Card(Three, Spade), Card(Four, Club), Card(Six, Diamond)]).find_highest_comb(),
            Comb::new(CombType::Straight, Six)
        );
        assert_eq!(
            Hand(vec![Card(Ten, Heart), Card(Six, Heart), Card(Ace, Heart), Card(Four, Heart), Card(Queen, Heart)]).find_highest_comb(),
            Comb::new(CombType::Flush, Ace)
        );
        assert_eq!(
            Hand(vec![Card(Jack, Spade), Card(Jack, Diamond), Card(Three, Spade), Card(Three, Club), Card(Three, Diamond)]).find_highest_comb(),
            Comb::new(CombType::FullHouse, Three)
        );
        assert_eq!(
            Hand(vec![Card(Ace, Diamond), Card(Six, Heart), Card(Ace, Heart), Card(Ace, Club), Card(Ace, Spade)]).find_highest_comb(),
            Comb::new(CombType::FourOfAKind, Ace)
        );
        assert_eq!(
            Hand(vec![Card(Eight, Club), Card(Jack, Club), Card(Seven, Club), Card(Ten, Club), Card(Nine, Club)]).find_highest_comb(),
            Comb::new(CombType::StraightFlush, Jack)
        );
    }

    #[test]
    fn hand_valid_tests() {
        assert!(Hand(vec![Card(Six, Club), Card(King, Heart), Card(Queen, Heart), Card(Four, Spade), Card(Ten, Club)]).is_valid());
        assert!(!Hand(vec![Card(Two, Diamond), Card(Ten, Club), Card(Ten, Spade), Card(Jack, Heart), Card(Queen, Club), Card(Ace, Heart)]).is_valid());
        assert!(!Hand(vec![Card(Eight, Club), Card(Jack, Club), Card(Seven, Club), Card(Ten, Club), Card(Eight, Club)]).is_valid())
    }
}
