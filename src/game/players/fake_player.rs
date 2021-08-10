use super::player::Player;
use crate::game;
use game::cards::{BlackCard, Deck, WhiteCard};
use std::collections::HashMap;

pub(crate) struct FakePlayer {
    pub name: String,
    hand: Vec<WhiteCard>,
}

impl FakePlayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::with_capacity(10),
        }
    }
}

impl Player for FakePlayer {
    fn get_name(self) -> String {
        self.name
    }

    fn fill_hand(&mut self, deck: &mut Deck<WhiteCard>) {
        let to_draw = 10 - self.hand.len();
        let mut drawn_cards = deck.draw_many(to_draw).unwrap();
        self.hand.append(&mut drawn_cards);
    }

    fn get_choice(&mut self, prompt: &BlackCard) -> Vec<WhiteCard> {
        self.hand.drain(0..prompt.pick as usize).collect()
    }

    fn choose_round_winner<'a>(&self, responses: HashMap<&'a str, Vec<WhiteCard>>) -> &'a str {
        let keys: Vec<&str> = responses.keys().cloned().collect();
        keys.get(0).unwrap()
    }
}
