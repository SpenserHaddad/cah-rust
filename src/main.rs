mod game;

use game::cards::{load_cards};

fn main() -> Result<(), ()> {
    let (mut black_cards, mut white_cards) = load_cards();
    println!("Found {} black cards!", black_cards.len());
    println!("Found {} white cards!", white_cards.len());
    black_cards.shuffle();
    for i in 0..10 {
        println!("{} - {}", i, black_cards.draw())
    }
    print!("\n");
    white_cards.shuffle();
    for i in 0..10 {
        println!("{} - {}", i, white_cards.draw())
    }
    Ok(())
}
