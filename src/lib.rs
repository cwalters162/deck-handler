#![allow(dead_code)]
struct Card {
    suit: &'static str,
}

impl Card {
    fn new() -> Self {
        Self {
            suit: "Diamond"
        }
    }

    fn get_suit(&self) -> &'static str {
        self.suit
    }
}

#[cfg(test)]
mod tests {
    use crate::Card;

    #[test]
    fn create_new_card() {
        let _card = Card::new();
    }

    #[test]
    fn get_card_suit() {
        let card = Card::new();
        let suit = card.get_suit();
        assert_eq!(suit, "Diamond");
    }
}
