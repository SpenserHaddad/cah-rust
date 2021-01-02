use game::cards::WhiteCard;

use crate::game;

pub struct Player {
    name: String,
    score: u8,
    hand: Vec<WhiteCard>,
}
