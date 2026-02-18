use core::GameState;
use core::SideCard;
use agents::{Agent, RandomAgent};
use rand;

fn play_round(mut state: GameState, a1: &mut dyn Agent, a2: &mut dyn Agent) -> (GameState, Option<i8>) {
    loop {
        if state.is_round_over() {
            break;
        }

        if state.current_player().stood {
            state.next_turn();
            continue;
        }

        state.start_turn_if_needed();

        if state.is_round_over() {
            break;
        }
        
        let legal = state.legal_actions();
        if legal.is_empty() {
            state.next_turn();
            continue;
        }

        // escolhe ação pelo agente correspondente
        let action = if state.player_turn {
            a1.select_action(&state)
        } else {
            a2.select_action(&state)
        };

        state.apply_action(action);
    }
    let winner = state.round_winner();
    (state, winner)
}

fn play_match(a1: &mut dyn Agent, a2: &mut dyn Agent) -> i8 {
    // best of 5
    let mut score1 = 0;
    let mut score2 = 0;
    // creates pools with 10 side cards each
    let pool1 = SideCard::random_pool(10);
    let pool2 = SideCard::random_pool(10);
    let init_state = GameState::new(pool1, pool2);

    let mut hand1 = init_state.player.side_hand.clone();
    let mut hand2 = init_state.opponent.side_hand.clone();
    // let mut player1_starts = true;
    let mut player1_starts = rand::random::<bool>(); // random start
    while score1 < 3 && score2 < 3 {
        let mut state = GameState::new_with_hands(hand1.clone(), hand2.clone());
        if player1_starts {
            state.player_turn = true;
        } 
        else {
            state.player_turn = false;
        }        
        let (end_state, result) = play_round(state, a1, a2);
        hand1 = end_state.player.side_hand.clone();
        hand2 = end_state.opponent.side_hand.clone();
        player1_starts = !player1_starts;
        match result {
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