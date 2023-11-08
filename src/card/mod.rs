mod test;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn random() -> Self {
        Self {
            suit: rand::random(),
            rank: rand::random(),
        }
    }

    pub fn get_suit(&self) -> Suit {
        self.suit
    }

    pub fn get_rank(&self) -> Rank {
        self.rank
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Suit {
    Diamonds,
    Spades,
    Clovers,
    Hearts,
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.gen_range(0..=3) {
            0 => Suit::Spades,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Clovers,
            _ => Suit::Spades,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank {
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

impl Distribution<Rank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        match rng.gen_range(0..=13) {
            0 => Rank::Ace,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace,
        }
    }
}
