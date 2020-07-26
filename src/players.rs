use super::cards::WhiteCard;

pub struct Player {
    name: String,
    score: u8,
    hand: Vec<WhiteCard>,
}
