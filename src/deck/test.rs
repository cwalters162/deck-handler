use crate::card::Card;
use crate::card::Rank::*;
use crate::card::Suit::*;
#[cfg(test)]
use crate::deck::Deck;

#[test]
fn can_create_deck() {
    let _deck = Deck::default();
}

#[test]
fn can_create_default_deck() {
    let _deck = Deck::default();
}

#[test]
fn default_deck_contains_52_cards() {
    let deck = Deck::default();
    assert_eq!(deck.remaining_cards(), 52)
}

#[test]
fn can_draw_card_from_the_top_of_the_deck() {
    let mut deck = Deck::default();
    let _card = deck.draw();
}

#[test]
fn can_draw_card_from_default_deck_and_have_51_remaning() {
    let mut deck = Deck::default();
    let _card = deck.draw();
    let cards_remaning = deck.remaining_cards();
    assert_eq!(cards_remaning, 51);
}

#[test]
fn default_deck_contains_one_of_each_card_rank_suit_combo_and_only_those() {
    let mut deck = Deck::default();

    let mut expected_default_deck = make_expected_default_deck();

    for _ in 0..=52 {
        let expected_card = expected_default_deck.pop();
        let result_card = deck.draw();
        assert_eq!(result_card, expected_card);
    }

    assert_eq!(deck.draw(), None)
}

#[test]
fn can_shuffle_the_deck() {
    let mut deck = Deck::default();
    deck.shuffle();
}


fn make_expected_default_deck() -> Vec<Card> {
    vec![
        Card::new(Spades, Ace),
        Card::new(Spades, Two),
        Card::new(Spades, Three),
        Card::new(Spades, Four),
        Card::new(Spades, Five),
        Card::new(Spades, Six),
        Card::new(Spades, Seven),
        Card::new(Spades, Eight),
        Card::new(Spades, Nine),
        Card::new(Spades, Ten),
        Card::new(Spades, Jack),
        Card::new(Spades, Queen),
        Card::new(Spades, King),

        Card::new(Diamonds, Ace),
        Card::new(Diamonds, Two),
        Card::new(Diamonds, Three),
        Card::new(Diamonds, Four),
        Card::new(Diamonds, Five),
        Card::new(Diamonds, Six),
        Card::new(Diamonds, Seven),
        Card::new(Diamonds, Eight),
        Card::new(Diamonds, Nine),
        Card::new(Diamonds, Ten),
        Card::new(Diamonds, Jack),
        Card::new(Diamonds, Queen),
        Card::new(Diamonds, King),

        Card::new(Hearts, Ace),
        Card::new(Hearts, Two),
        Card::new(Hearts, Three),
        Card::new(Hearts, Four),
        Card::new(Hearts, Five),
        Card::new(Hearts, Six),
        Card::new(Hearts, Seven),
        Card::new(Hearts, Eight),
        Card::new(Hearts, Nine),
        Card::new(Hearts, Ten),
        Card::new(Hearts, Jack),
        Card::new(Hearts, Queen),
        Card::new(Hearts, King),

        Card::new(Clovers, Ace),
        Card::new(Clovers, Two),
        Card::new(Clovers, Three),
        Card::new(Clovers, Four),
        Card::new(Clovers, Five),
        Card::new(Clovers, Six),
        Card::new(Clovers, Seven),
        Card::new(Clovers, Eight),
        Card::new(Clovers, Nine),
        Card::new(Clovers, Ten),
        Card::new(Clovers, Jack),
        Card::new(Clovers, Queen),
        Card::new(Clovers, King),
    ]
}