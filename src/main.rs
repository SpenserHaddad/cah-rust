mod game;

use game::cards::load_from_json;
use game::cards::Deck;

fn main() -> Result<(), ()> {
    let card_json_bytes = include_str!("cards.json");
    let (black_cards, white_cards) = load_from_json(card_json_bytes).unwrap();
    let mut black_deck = Deck::new(black_cards.clone()).unwrap();
    let mut white_deck = Deck::new(white_cards.clone()).unwrap();

    println!("Found {} black cards!", black_deck.len());
    println!("Found {} white cards!", white_deck.len());
    black_deck.shuffle();
    let black_hand = black_deck.draw_many(10).unwrap();
    for (idx, bc) in black_hand.iter().enumerate() {
        println!("{} - {}", idx, bc)
    }
    print!("\n");
    white_deck.shuffle();
    let white_hand = white_deck.draw_many(10).unwrap();
    for (idx, wc) in white_hand.iter().enumerate() {
        println!("{} - {}", idx, wc)
    }
    Ok(())
}
