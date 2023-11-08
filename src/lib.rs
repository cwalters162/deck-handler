#![allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Suit {
    Diamond,
    Spade
}

struct Card {
    suit: Suit,
}

impl Card {
    fn new(suit: Suit) -> Self {
        Self {
            suit
        }
    }

    fn get_suit(&self) -> Suit {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Suit};

    #[test]
    fn create_new_card() {
        let _card = Card::new(Suit::Diamond);
    }

    #[test]
    fn get_card_suit() {
        let card = Card::new(Suit::Diamond);
        let _suit = card.get_suit();
    }

    #[test]
    fn get_card_suit_matching_constructor() {
        let card = Card::new(Suit::Diamond);
        let suit = card.get_suit();
        assert_eq!(suit, Suit::Diamond)
    }

    #[test]
    fn create_spades_card() {
        let card = Card::new(Suit::Spade);
        assert_eq!(card.suit, Suit::Spade)
    }
}
