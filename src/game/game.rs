use std::collections::HashMap;

use crate::game;

use game::cards::Deck;
use game::cards::{BlackCard, WhiteCard};
use game::players::Player;
use game::settings::GameSettings;

pub struct CahGame<'cah> {
    players: Vec<Box<dyn Player + 'cah>>,
    judge_player_index: usize,
    config: GameSettings,
    black_deck: Deck<BlackCard>,
    white_deck: Deck<WhiteCard>,
    scores: HashMap<String, u8>,
}

impl CahGame {
    pub fn new(
        config: GameSettings,
        black_deck: Deck<BlackCard>,
        white_deck: Deck<WhiteCard>,
    ) -> Self {
        let player_count = config.player_count as usize;
        Self {
            config,
            black_deck,
            white_deck,
            judge_player_index: 0,
            players: Vec::with_capacity(player_count),
            scores: HashMap::with_capacity(player_count),
        }
    }

    pub fn add_player(&mut self, player: impl Player + 'cah) -> std::result::Result<(), &str> {
        if self.players.len() > self.config.player_count as usize {
            return Err("Game is full");
        }
        let player_name = player.get_name().clone();
        self.players.push(Box::new(player));
        self.scores.insert(player_name, 0);
        Ok(())
    }

    pub fn setup_game(&mut self) {
        for player in self.players.iter_mut() {
            player.fill_hand(&mut self.white_deck);
        }
    }

    pub fn play_turn(&mut self) {
        let mut player_choices: HashMap<&str, Vec<WhiteCard>> = HashMap::new();
        let judge = self.players.remove(self.judge_player_index);
        let prompt_card = self.black_deck.draw().unwrap();
        for player in self.players.iter_mut() {
            let player_choice = player.get_choice(&prompt_card);
            player_choices.insert(&player.get_name(), player_choice);
        }

        let winner = judge.choose_round_winner(player_choices);
        self.scores
            .insert(winner.to_owned(), self.scores[winner] + 1);
        self.players.insert(self.judge_player_index, judge);
    }
}
