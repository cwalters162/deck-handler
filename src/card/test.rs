#[cfg(test)]
mod card_tests {
    use crate::card::card::{Card, Rank, Suit};

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
        assert_eq!(card.get_suit(), Suit::Spades)
    }

    #[test]
    fn create_clover_card() {
        let card = Card::new(Suit::Clovers, Rank::Ace);
        assert_eq!(card.get_suit(), Suit::Clovers)
    }

    #[test]
    fn create_hearts_card() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(card.get_suit(), Suit::Hearts)
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