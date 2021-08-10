use std::collections::HashMap;

use game::cards::WhiteCard;
use game::cards::{BlackCard, Deck};

use crate::game;

const HAND_SIZE: usize = 10;

pub trait Player {
    fn fill_hand(&mut self, deck: &mut Deck<WhiteCard>) -> ();
    fn get_choice(&mut self, prompt: &BlackCard) -> Vec<WhiteCard>;
    fn choose_round_winner<'a>(&self, responses: HashMap<&'a str, Vec<WhiteCard>>) -> &'a str;
}

pub struct SamplePlayer {
    pub name: String,
    hand: Vec<WhiteCard>,
}

impl SamplePlayer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hand: Vec::with_capacity(HAND_SIZE),
        }
    }
}

impl Player for SamplePlayer {
    fn fill_hand(&mut self, deck: &mut Deck<WhiteCard>) {
        let to_draw = HAND_SIZE - self.hand.len();
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
