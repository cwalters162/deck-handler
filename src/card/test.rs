#[cfg(test)]
use crate::card::{Card, Rank, Rank::*, Suit, Suit::*};

#[test]
fn create_new_card() {
    let _card = Card::new(Diamonds, Ace);
}

#[test]
fn create_random_card() {
    let _card = Card::random();
}

#[test]
fn get_card_suit() {
    let card = Card::random();
    let _suit = card.suit();
}

#[test]
fn get_card_suit_matching_constructor() {
    let card = Card::new(Spades, Ace);
    let suit = card.suit();
    assert_eq!(suit, Spades);
}

#[test]
fn create_spades_card() {
    let card = Card::new(Spades, Ace);
    assert_eq!(card.suit(), Spades)
}

#[test]
fn create_clover_card() {
    let card = Card::new(Clovers, Ace);
    assert_eq!(card.suit(), Clovers)
}

#[test]
fn create_hearts_card() {
    let card = Card::new(Hearts, Ace);
    assert_eq!(card.suit(), Hearts)
}

#[test]
fn create_diamonds_card() {
    let card = Card::new(Diamonds, Ace);
    assert_eq!(card.suit(), Diamonds)
}

#[test]
fn get_card_rank() {
    let card = Card::random();
    let _rank = card.rank();
}

#[test]
fn rank_matches_constructor() {
    let card = Card::new(Spades, Ace);
    let rank = card.rank();
    assert_eq!(rank, Ace)
}
#[test]
fn card_can_be_all_ranks() {
    impl Rank {
        fn iterate() -> impl Iterator<Item = Rank> {
            [
                Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
            ]
            .iter()
            .copied()
        }
    }

    for rank in Rank::iterate() {
        let card = Card::new(Spades, rank);
        assert_eq!(card, Card::new(Spades, rank));
    }
}

#[test]
fn card_can_be_all_suits() {
    impl Suit {
        fn iterate() -> impl Iterator<Item = Suit> {
            [Diamonds, Hearts, Spades, Clovers].iter().copied()
        }
    }
    for suit in Suit::iterate() {
        let card = Card::new(suit, Ace);
        assert_eq!(card, Card::new(suit, Ace))
    }
}
