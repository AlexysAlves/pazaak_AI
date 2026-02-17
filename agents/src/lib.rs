use rand::{Rng, rngs::ThreadRng, thread_rng};
use core::GameState;
use core::Action;

pub trait Agent {
    fn select_action(&mut self, state: &GameState) -> Action;
}

pub struct RandomAgent {
    rng: ThreadRng,
}

impl RandomAgent {
    pub fn new() -> Self { Self { rng: thread_rng() } }
}

impl Agent for RandomAgent {
    fn select_action(&mut self, state: &GameState) -> Action {
        let legal = state.legal_actions();
        if legal.is_empty() {
            return Action::Draw;
        }
        let idx = self.rng.gen_range(0..legal.len());
        legal[idx]
    }
}