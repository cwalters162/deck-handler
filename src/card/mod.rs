mod test;

mod card {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum Suit {
        Diamonds,
        Spades,
        Clovers,
        Hearts,
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

    pub struct Card {
        suit: Suit,
        rank: Rank,
    }

    impl Card {

        pub fn new(suit: Suit, rank: Rank) -> Self {
            Self { suit, rank }
        }

        pub fn get_suit(&self) -> Suit {
            self.suit
        }

        pub fn get_rank(&self) -> Rank {
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
}