mod game;

use game::cards::{load_cards, BlackCard, WhiteCard};
use game::players::Player;
use rand::distributions::{Distribution, Uniform};

fn main() -> Result<(), ()> {
    let (mut black_cards, mut white_cards) = load_cards();
    println!("Found {} black cards!", black_cards.len());
    println!("Found {} white cards!", white_cards.len());
    shuffle_deck(&mut black_cards);
    for i in 0..10 {
        println!("{} - {}", i, black_cards[i])
    }
    print!("\n");
    shuffle_deck(&mut white_cards);
    for i in 0..10 {
        println!("{} - {}", i, white_cards[i])
    }
    Ok(())
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
