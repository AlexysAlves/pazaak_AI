use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Stand,
    PlaySide(usize, Option<bool>),
    EndTurn,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub score: i8,             // current sum
    pub side_deck_all: Vec<SideCard>, // all 10 initial cards
    pub side_pool: Vec<SideCard>, // all 10 cards (remaining in pool after drawing initial hand)
    pub side_hand: Vec<SideCard>, // active 4 cards (can be played)
    pub played_cards: Vec<CardEvent>, // card history
    pub used_side_this_turn: bool,
    pub has_drawn_this_turn: bool,
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

#[derive(Debug, Clone)]
pub enum SideCard {
    Simple(i8),   // value already defined
    Flip(i8),     // magnitude and sign
}

#[derive(Debug, Clone)]
pub enum CardEvent {
    MainDeck(i8),
    SideDeck(SideCard),
}
fn build_player_state(
    side_deck_all: Vec<SideCard>,
    side_pool: Vec<SideCard>,
    side_hand: Vec<SideCard>,
    played_cards: Vec<CardEvent>,
) -> PlayerState {
    PlayerState {
        score: 0,
        side_deck_all,
        side_pool,
        side_hand,
        played_cards,
        used_side_this_turn: false,
        has_drawn_this_turn: false,
        stood: false,
    }
}

impl GameState {
    pub fn new(side_pool_player: Vec<SideCard>, side_pool_opponent: Vec<SideCard>) -> Self {
        let mut rng = thread_rng();

        let mut side_deck_all_p = side_pool_player.clone();
        side_deck_all_p.shuffle(&mut rng);
        // clone pools and sample 4 for each hand 
        let mut pool_p = side_deck_all_p.clone();
        let hand_p: Vec<SideCard> = pool_p.drain(0..4.min(pool_p.len())).collect();

        let mut side_deck_all_o = side_pool_opponent.clone();
        side_deck_all_o.shuffle(&mut rng);

        let mut pool_o = side_deck_all_o.clone();
        let hand_o: Vec<SideCard> = pool_o.drain(0..4.min(pool_o.len())).collect();
        Self {
            player: PlayerState {
                score: 0,
                side_deck_all: side_deck_all_p,
                side_pool: pool_p,
                side_hand: hand_p,
                played_cards: vec![],
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            opponent: PlayerState {
                score: 0,
                side_deck_all: side_deck_all_o,
                side_pool: pool_o,
                side_hand: hand_o,
                played_cards: vec![],
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            player_turn: true,
            deck: Deck::new_shuffled(),
        }
    }
    pub fn new_with_match_state(
        player_side_deck_all: Vec<SideCard>,
        player_side_pool: Vec<SideCard>,
        player_side_hand: Vec<SideCard>,
        player_played_cards: Vec<CardEvent>,
        opponent_side_deck_all: Vec<SideCard>,
        opponent_side_pool: Vec<SideCard>,
        opponent_side_hand: Vec<SideCard>,
        opponent_played_cards: Vec<CardEvent>,
        player_turn: bool,
    ) -> Self {
        Self {
            player: build_player_state(
                player_side_deck_all,
                player_side_pool,
                player_side_hand,
                player_played_cards,
            ),
            opponent: build_player_state(
                opponent_side_deck_all,
                opponent_side_pool,
                opponent_side_hand,
                opponent_played_cards,
            ),
            player_turn,
            deck: Deck::new_shuffled(),
        }
    }
    /// returns legal actions at current state
    pub fn legal_actions(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.current_player().stood{
            return actions;
        }
        actions.push(Action::Stand);
        actions.push(Action::EndTurn);
        // side play only if player hasn't used side this turn
        if !self.current_player().used_side_this_turn {
            for (idx, card) in self.current_player().side_hand.iter().enumerate() {
                match card {
                    SideCard::Simple(_) => actions.push(Action::PlaySide(idx, None)),
                    SideCard::Flip(_) => {
                        actions.push(Action::PlaySide(idx, Some(true)));
                        actions.push(Action::PlaySide(idx, Some(false)));
                    }
                }
            }
        }
        //else {
          //  actions.push(Action::Draw);
       // }
        actions
    }

    pub fn current_player(&self) -> &PlayerState {
        if self.player_turn { &self.player } else { &self.opponent }
    }

    pub fn current_player_mut(&mut self) -> &mut PlayerState {
        if self.player_turn { &mut self.player } else { &mut self.opponent }
    }
    
    /// apply an action and advance state
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Stand => {
                let p = self.current_player_mut();
                p.stood = true;
            }
            Action::PlaySide(idx, flip_choice) => {
                let played_card = {
                    self.current_player().side_hand[idx].clone()
                };
                
                // play side card at index in hand 
                let value: i8 = {
                    match played_card.clone() {
                        SideCard::Simple(v) => v,
                        SideCard::Flip(mag) => {
                            match flip_choice {
                                Some(true) => mag,
                                Some(false) => -mag,
                                None => panic!("Flip played without flip_choice"),
                            }
                        }
                    }
                    
                };
                {
                    let p = self.current_player_mut();
                    p.side_hand.remove(idx);
                    p.score += value;
                    p.played_cards.push(CardEvent::SideDeck(played_card));
                    p.used_side_this_turn = true;
                }
            }
            Action::EndTurn => {}
        }
        // change turn
        if !self.is_round_over() {
            self.next_turn();
        }
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

    /// Draws automatically
    pub fn start_turn_if_needed(&mut self) {
        // doenst draw if has stood
        if self.current_player().stood {
            return;
        }

        if self.current_player().has_drawn_this_turn {
            return;
        }

        let card = self.deck.draw();
        {
            let p = self.current_player_mut();
            p.score += card;
            p.played_cards.push(CardEvent::MainDeck(card));
            p.has_drawn_this_turn = true;
        }
    }

    pub fn next_turn(&mut self) {
        self.player_turn = !self.player_turn;
    
        let p = self.current_player_mut();
        p.used_side_this_turn = false;
        p.has_drawn_this_turn = false;
        self.start_turn_if_needed();
    }

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

impl SideCard {
    /// helper: build a random pool of length n
    pub fn random_pool(n: usize) -> Vec<SideCard> {
        let mut rng = thread_rng();
        let mut pool: Vec<SideCard> = Vec::with_capacity(n);
        for _ in 0..n {
            let r: f32 = rand::random();
            if r < 0.4 {
                // positive card magnitude 1..6
                let mag = (rng.gen_range(1..=6)) as i8;
                pool.push(SideCard::Simple(mag));
            } else if r < 0.8 {
                let mag = (rng.gen_range(1..=6)) as i8;
                pool.push(SideCard::Simple(-mag));
            } else {
                let mag = (rng.gen_range(1..=6)) as i8;
                pool.push(SideCard::Flip(mag));
            }
        }
        pool
    }
}