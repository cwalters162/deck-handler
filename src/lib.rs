struct Card {

}

impl Card {
    fn new() -> Card {
        Card {}
    }
}

#[cfg(test)]
mod tests {
    use crate::Card;

    #[test]
    fn create_new_card() {
        let card = Card::new();
    }
}
