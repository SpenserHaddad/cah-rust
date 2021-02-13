use std::fmt::Debug;

use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub enum DeckError {
    NoCards,
    InvalidCards,
}

#[derive(Debug)]
pub struct Deck<T>
where
    T: Debug,
{
    draw_pile: Vec<T>,
    active_cards: Vec<T>,
    discard_pile: Vec<T>,
    deck_size: usize,
}

impl<T> Deck<T>
where
    T: Clone + Debug,
{
    pub fn new(cards: Vec<T>) -> Result<Self, DeckError> {
        if cards.is_empty() {
            return Err(DeckError::InvalidCards);
        }
        let deck_size = cards.len();
        let new_deck = Self {
            draw_pile: cards,
            active_cards: Vec::new(),
            discard_pile: Vec::new(),
            deck_size,
        };
        Ok(new_deck)
    }

    pub fn draw(&mut self) -> Result<T, DeckError> {
        if self.draw_pile.is_empty() {
            if !self.discard_pile.is_empty() {
                self.shuffle()
            } else {
                return Err(DeckError::NoCards);
            }
        }
        let drawn_card = self.draw_pile.remove(0);
        Ok(drawn_card)
    }

    pub fn draw_many(&mut self, count: usize) -> Result<Vec<T>, DeckError> {
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
        let int_deck = Deck::new(ints).unwrap();
        assert_eq!(int_deck.len(), 5);
    }

    #[test]
    fn test_create_deck_empty_deck_returns_err() {
        let empty_vec: Vec<i32> = Vec::new();
        match Deck::new(empty_vec) {
            Ok(_deck) => assert!(false),
            Err(_de) => assert!(true),
        }
    }

    #[test]
    fn test_draw_returns_correct_value() {
        let ints = vec![1, 2, 3, 4, 5];
        let mut int_deck = Deck::new(ints).unwrap();
        let drawn_int = int_deck.draw().unwrap();
        assert_eq!(drawn_int, 1);
    }

    #[test]
    fn test_draw_no_cards_returns_err() {
        let ints = vec![0];
        let mut int_deck = Deck::new(ints).unwrap();
        let drawn_int = int_deck.draw().unwrap();
        assert_eq!(drawn_int, 0);

        let result = int_deck.draw();
        assert_eq!(result, Err(DeckError::NoCards));
    }

    #[test]
    fn test_draw_many_returns_values() {
        let ints = vec![1, 2, 3, 4, 5];
        let expected_ints = vec![1, 2, 3];
        let mut int_deck = Deck::new(ints).unwrap();
        let drawn_ints = int_deck.draw_many(3).unwrap();
        assert_eq!(drawn_ints, expected_ints);
    }

    #[test]
    fn test_draw_many_too_many_cards_returns_err() {
        let ints = vec![1, 2, 3, 4, 5];
        let mut int_deck = Deck::new(ints).unwrap();
        match int_deck.draw_many(10) {
            Ok(_cards) => assert!(false),
            Err(_de) => assert!(true),
        }
    }
}
