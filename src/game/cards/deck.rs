use rand::seq::SliceRandom;

pub struct Deck<T> where T: Copy {
    draw_pile: Vec<T>,
    active_cards: Vec<T>,
    discard_pile: Vec<T>,
    deck_size: usize,
}

impl<T> Deck<T> where T: Copy {
    pub fn new(cards: Vec<T> ) -> Self {
        Self {
            draw_pile: cards,
            active_cards: Vec::new(),
            discard_pile: Vec::new(),
            deck_size: cards.len(),
        }
    }

    pub fn draw(&mut self) -> &T {
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

    pub fn shuffle(&mut self) {
        self.draw_pile.append(&mut self.discard_pile);
        let mut rng = rand::thread_rng();
        self.draw_pile.shuffle(&mut rng);
    }

    fn discard(&mut self, card: T) {
        self.discard_pile.push(card);
    }

    pub fn len(&self) -> usize {
        self.deck_size
    }
}
