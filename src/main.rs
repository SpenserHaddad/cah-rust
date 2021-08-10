mod game;

use game::settings::GameSettings;
use game::{cards::load_from_json, players::SamplePlayer};
use game::{cards::Deck, game::CahGame};

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

    let game_config = GameSettings {
        player_count: 4,
        winning_score: 10,
    };
    let players: Vec<SamplePlayer> = vec![
        SamplePlayer::new("Player1".to_owned()),
        SamplePlayer::new("Player2".to_owned()),
    ];
    let mut game = CahGame::new(game_config, black_deck, white_deck);
    for p in players {
        game.add_player(p).unwrap();
    }
    game.setup_game();
    game.play_turn();
    Ok(())
}
