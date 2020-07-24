use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fmt::{Display, Formatter};

fn main() -> Result<()> {
    let card_data = include_bytes!("cards.json");
    let card_data: Value = serde_json::from_slice(card_data).unwrap();
    let card_data = card_data.as_object().unwrap();

    let mut black_cards: Vec<BlackCard> =
        serde_json::from_value(card_data["blackCards"].to_owned()).unwrap();
    let mut white_cards: Vec<WhiteCard> = card_data["whiteCards"]
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
    println!("Found {} black cards!", black_cards.len());
    println!("Found {} white cards!", white_cards.len());
    shuffle_deck(&mut black_cards);
    for i in 0..10 {
        println!("{} - {}", i, black_cards[i])
    }
    print!("\n");
    shuffle_deck(&mut white_cards);
    for i in 0..10 {
        println!("{} - {:?}", i, white_cards[i])
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct BlackCard {
    text: String,
    pick: u8,
}

impl Display for BlackCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\" (Pick {})", self.text, self.pick)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WhiteCard {
    text: String,
}

impl Display for WhiteCard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "\"{}\"", self.text)
    }
}

fn shuffle_deck<T>(deck: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    // let range = Uniform::from(0..(deck.len()));
    for i in (0..deck.len()).rev() {
        let range = Uniform::new_inclusive(0, i);
        let j: usize = range.sample(&mut rng);
        deck.swap(i, j);
    }
}

struct Player {
    name: String,
    score: u8,
    hand: Vec<WhiteCard>,
}

struct CardsAgainstHumanityGame {
    white_deck: Vec<WhiteCard>,
    black_deck: Vec<BlackCard>,
    players: Vec<Player>,
}

struct Deck<T> {
    draw_pile: Vec<T>,
    active_cards: Vec<T>,
    discard_pile: Vec<T>,
}

impl<T> Deck<T> {
    fn draw(&mut self) -> &T {
        let drawn_card = match self.draw_pile.pop() {
            Some(card) => card,
            None => {
                self.shuffle();
                self.draw_pile.pop().unwrap()
            }
        };

        self.active_cards.push(drawn_card);
        self.active_cards.last().unwrap()
    }

    fn shuffle(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        let mut rng = rand::thread_rng();
        for i in (0..self.draw_pile.len()).rev() {
            let range = Uniform::new_inclusive(0, i);
            let j: usize = range.sample(&mut rng);
            self.draw_pile.swap(i, j);
        }
    }

    fn discard(&mut self, card: T) {
        self.discard_pile.push(card);
    }
}
