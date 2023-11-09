mod test;
pub struct Deck {}

impl Default for Deck {
    fn default() -> Self {
        Self {}
    }
}

impl Deck {
    pub fn remaining_cards(&self) -> usize {
        52
    }
}
