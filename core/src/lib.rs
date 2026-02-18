use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Stand,
    PlaySide(usize, Option<bool>),
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub score: i8,             // current sum
    pub side_pool: Vec<SideCard>, // all 10 cards (remaining in pool after drawing initial hand)
    pub side_hand: Vec<SideCard>, // active 4 cards (can be played)
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

impl GameState {
    pub fn new(side_pool_player: Vec<SideCard>, side_pool_opponent: Vec<SideCard>) -> Self {
        let mut rng = thread_rng();

        // clone pools and sample 4 for each hand 
        let mut pool_p = side_pool_player.clone();
        pool_p.shuffle(&mut rng);
        let hand_p: Vec<SideCard> = pool_p.drain(0..4.min(pool_p.len())).collect();

        let mut pool_o = side_pool_opponent.clone();
        pool_o.shuffle(&mut rng);
        let hand_o: Vec<SideCard> = pool_o.drain(0..4.min(pool_o.len())).collect();
        Self {
            player: PlayerState {
                score: 0,
                side_pool: pool_p,
                side_hand: hand_p,
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            opponent: PlayerState {
                score: 0,
                side_pool: pool_o,
                side_hand: hand_o,
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            player_turn: true,
            deck: Deck::new_shuffled(),
        }
    }
    pub fn new_with_hands(player_hand: Vec<SideCard>, opponent_hand: Vec<SideCard>) -> Self {
        Self {
            player: PlayerState {
                score: 0,
                side_pool: vec![],
                side_hand: player_hand,
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            opponent: PlayerState {
                score: 0,
                side_pool: vec![],
                side_hand: opponent_hand,
                used_side_this_turn: false,
                has_drawn_this_turn: false, 
                stood: false,
            },
            player_turn: rand::random::<bool>(), // deixa justo
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
    
    pub fn other_player_mut(&mut self) -> &mut PlayerState {
        if self.player_turn { &mut self.opponent } else { &mut self.player }
    }

    /// apply an action and advance state
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Stand => {
                let p = self.current_player_mut();
                p.stood = true;
            }
        //    Action::Draw => {
          //      let card = self.deck.draw();
            //    let p = self.current_player_mut();
              //  p.score += card;
            //}
            Action::PlaySide(idx, flip_choice) => {
                // play side card at index in hand 
                let value: i8 = {
                    let hand_len = self.current_player().side_hand.len();
                    if idx >= hand_len {
                        // shouldnt happen
                        0
                    } 
                    else {
                        match self.current_player().side_hand[idx].clone() {
                            SideCard::Simple(v) => v,
                            SideCard::Flip(mag) => {
                                match flip_choice {
                                    Some(true) => mag,
                                    Some(false) => -mag,
                                    None => {
                                        mag
                                    }
                                }
                            }
                        }
                    }
                };
                {
                    let p = self.current_player_mut();
                    if idx < p.side_hand.len() {
                        p.side_hand.remove(idx);
                    }
                    p.score += value;
                    p.used_side_this_turn = true;
                }
            }
        }
        // change turn
        self.player_turn = !self.player_turn;

        let p_now = self.current_player_mut();
        p_now.used_side_this_turn = false;
        p_now.has_drawn_this_turn = false;
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
            p.has_drawn_this_turn = true;
        }
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