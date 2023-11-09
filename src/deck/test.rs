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
