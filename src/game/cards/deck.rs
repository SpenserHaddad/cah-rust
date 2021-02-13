use rand::seq::SliceRandom;

pub struct Deck<T> {
    draw_pile: Vec<T>,
    active_cards: Vec<T>,
    discard_pile: Vec<T>,
    deck_size: usize,
}

impl<T> Deck<T>
where
    T: Clone,
{
    pub fn new(cards: Vec<T>) -> Self {
        let deck_size = cards.len();
        Self {
            draw_pile: cards,
            active_cards: Vec::new(),
            discard_pile: Vec::new(),
            deck_size,
        }
    }

    pub fn draw(&mut self) -> Result<T, &'static str> {
        if self.draw_pile.len() == 0 {
            self.shuffle()
        }
        let drawn_card = self.draw_pile.remove(0);
        Ok(drawn_card)
    }

    pub fn draw_many(&mut self, count: usize) -> Result<Vec<T>, &'static str> {
        let mut drawn_cards = Vec::with_capacity(count);
        for _ in 0..count {
            match self.draw() {
                Ok(drawn_card) => drawn_cards.push(drawn_card),
                Err(err) => return Err(err),
            };
        }
        Ok(drawn_cards)
    }

    pub fn shuffle(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        let mut rng = rand::thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }

    fn discard(&mut self, card: T) {
        self.discard_pile.push(card);
    }

    pub fn len(&self) -> usize {
        self.deck_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deck() {
        let ints = vec![1, 2, 3, 4, 5];
        let int_deck = Deck::new(ints);
        assert_eq!(int_deck.len(), 5)
    }

    #[test]
    fn test_draw() {
        let ints = vec![1, 2, 3, 4, 5];
        let mut int_deck = Deck::new(ints);
        let drawn_int = int_deck.draw().unwrap();
        assert_eq!(drawn_int, 1);
    }
}
