#![allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Suit {
    Diamond
}

struct Card {
    suit: Suit,
}

impl Card {
    fn new() -> Self {
        Self {
            suit: Suit::Diamond
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
        let _card = Card::new();
    }

    #[test]
    fn get_card_suit() {
        let card = Card::new();
        let _suit = card.get_suit();
    }
}
