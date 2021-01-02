use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use super::deck::Deck;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlackCard {
    text: String,
    pick: u8,
}

impl Display for BlackCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\" (Pick {})", self.text, self.pick)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhiteCard {
    text: String,
}

impl Display for WhiteCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.text)
    }
}

pub fn load_cards() -> (Deck<BlackCard>, Deck<WhiteCard>) {
    let card_json_bytes = include_bytes!("cards.json");
    let card_json: Value = serde_json::from_slice(card_json_bytes).unwrap();
    let card_data = card_json.as_object().unwrap();
    let black_cards: Vec<BlackCard> =
        serde_json::from_value(card_data["blackCards"].to_owned()).unwrap();

    let white_cards: Vec<WhiteCard> = card_data["whiteCards"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|c| {
            if let Value::String(card_text) = c {
                WhiteCard {
                    text: card_text.to_owned(),
                }
            } else {
                panic!("Found unexpected value parsing white cards: {:?}", c);
            }
        })
        .collect();

    let black_deck = Deck::new(black_cards);
    let white_deck = Deck::new(white_cards);
    (black_deck, white_deck)
}
