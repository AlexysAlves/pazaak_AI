use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Stand,
    Play(i8), // value from side deck card
    Pass,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub score: i8,             // current sum
    pub side_deck: Vec<i8>,    // available cards
    pub stood: bool,           // has/hasnt stood
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub player: PlayerState,
    pub opponent: PlayerState,
    pub player_turn: bool, // true/false = player/opponent
    pub deck: Deck, // next main deck card
}

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<i8>,
}


impl GameState {
    pub fn new(side_deck_player: Vec<i8>, side_deck_opponent: Vec<i8>) -> Self {
        Self {
            player: PlayerState { score: 0, side_deck: side_deck_player, stood: false },
            opponent: PlayerState { score: 0, side_deck: side_deck_opponent, stood: false },
            player_turn: true,
            deck: Deck::new_shuffled(),
        }
    }

    /// returns legal actions at current state
    pub fn legal_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if !self.current_player().stood 
        {
            actions.push(Action::Stand);
            actions.push(Action::Pass);
            for &c in &self.current_player().side_deck {
                actions.push(Action::Play(c));
            }
        } 
        else {
            actions.push(Action::Pass);
        }
        actions
    }

    fn current_player(&self) -> &PlayerState {
        if self.player_turn { &self.player } else { &self.opponent }
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        if self.player_turn { &mut self.player } else { &mut self.opponent }
    }

    /// apply an action and advance state
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Stand => {
                let p = self.current_player_mut();
                p.stood = true;
            }
            Action::Play(v) => {
                let p = self.current_player_mut();
                p.score += v;
                // remove first occurence
                if let Some(pos) = p.side_deck.iter().position(|&x| x == v) {
                    p.side_deck.remove(pos);
                }
                // doesnt change turn automatically
            }
            Action::Pass => {
                let card = self.deck.draw(); 
                let p = self.current_player_mut();
                p.score += card;
            }
        }
        // change turn
        self.player_turn = !self.player_turn;
    }

    pub fn is_round_over(&self) -> bool {
        // both stood or someone passed 20
        self.player.stood && self.opponent.stood || self.player.score > 20 || self.opponent.score > 20
    }

    pub fn round_winner(&self) -> Option<i8> {
        // returns Some(1) if player wins, Some(-1) if opponent wins, None if it is a tie
        if self.player.score > 20 && self.opponent.score > 20 { return None; }
        if self.player.score > 20 { return Some(-1); }
        if self.opponent.score > 20 { return Some(1); }
        if !self.is_round_over() { return None; }
        if self.player.score == self.opponent.score { return None; }
        if self.player.score.abs_diff(self.target()) < self.opponent.score.abs_diff(self.target()) { Some(1) } else { Some(-1) }
    }

    fn target(&self) -> i8 { 20 }
}

impl Deck {
    pub fn new_shuffled() -> Self {
        // 40 cards main deck
        let mut cards: Vec<i8> = Vec::new();
        for v in 1..=10 {
            for _ in 0..4 {
                cards.push(v);
            }
        }

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Self { cards }
    }

    pub fn draw(&mut self) -> i8 {
        // shuffle after deck is over
        if self.cards.is_empty() {
            *self = Self::new_shuffled();
        }

        self.cards.pop().unwrap()
    }
}
