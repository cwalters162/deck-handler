use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::Card;
use crate::card::Rank::*;
use crate::card::Suit::*;

mod test;
#[derive(PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            cards: vec![
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
            ],
        }
    }
}

impl Deck {
    pub fn remaining_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

}
