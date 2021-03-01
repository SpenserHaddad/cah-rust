use game::cards::Deck;
use game::cards::WhiteCard;

use crate::game;

const HAND_SIZE: usize = 10;

pub struct Player {
    name: String,
    score: u8,
    hand: Vec<WhiteCard>,
}

impl Player {
    pub fn fill_hand(&mut self, deck: &mut Deck<WhiteCard>) {
        let to_draw = HAND_SIZE - self.hand.len();
        let mut drawn_cards = deck.draw_many(to_draw).unwrap();
        self.hand.append(&mut drawn_cards);
    }
}
