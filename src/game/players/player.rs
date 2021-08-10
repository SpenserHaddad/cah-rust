use crate::game;
use game::cards::{BlackCard, Deck, WhiteCard};
use std::collections::HashMap;

pub const HAND_SIZE: usize = 10;

pub trait Player {
    fn get_name(self) -> String;
    fn fill_hand(&mut self, deck: &mut Deck<WhiteCard>) -> ();
    fn get_choice(&mut self, prompt: &BlackCard) -> Vec<WhiteCard>;
    fn choose_round_winner<'a>(&self, responses: HashMap<&'a str, Vec<WhiteCard>>) -> &'a str;
}
