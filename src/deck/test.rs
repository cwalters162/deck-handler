#[cfg(test)]
use crate::deck::Deck;

#[test]
fn can_create_deck() {
    let _deck = Deck {};
}

#[test]
fn can_create_default_deck() {
    let deck = Deck::default();
}

#[test]
fn default_deck_contains_52_cards() {
    let deck = Deck::default();
    assert_eq!(deck.remaining_cards(), 52)
}
