#![allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
enum Suit {
    Diamonds,
    Spades,
    Clovers,
    Hearts,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    fn get_suit(&self) -> Suit {
        self.suit
    }

    fn get_rank(&self) -> Rank {
        self.rank
    }
}

impl Default for Card {
    fn default() -> Card {
        Card {
            suit: Suit::Spades,
            rank: Rank::Ace,
        }
    }
}

#[cfg(test)]
mod card_tests {
    use crate::{Card, Rank, Suit};

    #[test]
    fn create_new_card() {
        let _card = Card::default();
    }

    #[test]
    fn get_card_suit() {
        let card = Card::default();
        let _suit = card.get_suit();
    }

    #[test]
    fn get_card_suit_matching_constructor() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        let suit = card.get_suit();
        assert_eq!(suit, Suit::Spades);
    }

    #[test]
    fn create_spades_card() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        assert_eq!(card.suit, Suit::Spades)
    }

    #[test]
    fn create_clover_card() {
        let card = Card::new(Suit::Clovers, Rank::Ace);
        assert_eq!(card.suit, Suit::Clovers)
    }

    #[test]
    fn create_hearts_card() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(card.suit, Suit::Hearts)
    }

    #[test]
    fn get_card_rank() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        let _rank = card.get_rank();
    }

    #[test]
    fn rank_matches_constructor() {
        let card = Card::new(Suit::Spades, Rank::Ace);
        let rank = card.get_rank();
        assert_eq!(rank, Rank::Ace)
    }
}
