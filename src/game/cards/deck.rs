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
