use core::GameState;
use core::Action;

pub trait Agent {
    fn select_action(&mut self, state: &GameState) -> Action;
}

pub struct RandomAgent {
    rng: rand::rngs::ThreadRng,
}

impl RandomAgent {
    pub fn new() -> Self { Self { rng: rand::thread_rng() } }
}

impl Agent for RandomAgent {
    fn select_action(&mut self, state: &GameState) -> Action {
        let legal = state.legal_actions();
        let idx = (self.rng.gen::<usize>() % legal.len()).min(legal.len()-1);
        legal[idx]
    }
}