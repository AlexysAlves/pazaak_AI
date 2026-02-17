use core::GameState;
use core::SideCard;
use agents::{Agent, RandomAgent};

fn play_round(mut state: GameState, a1: &mut dyn Agent, a2: &mut dyn Agent) -> Option<i8> {
    while !state.is_round_over() {
        if state.player_turn {
            let action = a1.select_action(&state);
            state.apply_action(action);
        } else {
            let action = a2.select_action(&state);
            state.apply_action(action);
        }
    }
    state.round_winner()
}

fn play_match(a1: &mut dyn Agent, a2: &mut dyn Agent) -> i8 {
    // best of 5
    let mut score1 = 0;
    let mut score2 = 0;
    while score1 < 3 && score2 < 3 {
        // creates pools with 10 side cards each
        let pool1 = SideCard::random_pool(10);
        let pool2 = SideCard::random_pool(10);

        // creates initial game state with 4 cards from each pool
        let state = GameState::new(pool1, pool2);
        match play_round(state, a1, a2) {
            Some(1) => score1 += 1,
            Some(-1) => score2 += 1,
            _ => { /* does nothing */ }
        }
    }
    if score1 >= 3 { 1 } else { -1 }
}

fn main() {
    // simple example
    let mut a1 = RandomAgent::new();
    let mut a2 = RandomAgent::new();
    let winner = play_match(&mut a1, &mut a2);
    println!("Match winner: {}", winner);
}