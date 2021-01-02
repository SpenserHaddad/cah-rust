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
