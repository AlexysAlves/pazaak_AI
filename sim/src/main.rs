use core::GameState;
use core::SideCard;
use agents::{Agent, RandomAgent};
use rand;

fn play_round(mut state: GameState, a1: &mut dyn Agent, a2: &mut dyn Agent) -> (GameState, Option<i8>) {
    
    state.start_turn_if_needed();
    loop {
        if state.is_round_over() {
            break;
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

    let mut player_state = init_state.player;
    let mut opponent_state = init_state.opponent;
    // let mut player1_starts = true;
    let mut player1_starts = rand::random::<bool>(); // random start
    while score1 < 3 && score2 < 3 {
        let state = GameState::new_with_match_state(
            player_state.side_deck_all.clone(),
            player_state.side_pool.clone(),
            player_state.side_hand.clone(),
            player_state.played_cards.clone(),
            opponent_state.side_deck_all.clone(),
            opponent_state.side_pool.clone(),
            opponent_state.side_hand.clone(),
            opponent_state.played_cards.clone(),
            player1_starts,
        );

        let (end_state, result) = play_round(state, a1, a2);

        player_state = end_state.player;
        opponent_state = end_state.opponent;
        player1_starts = !player1_starts;
        match result {
            Some(1) => score1 += 1,
            Some(-1) => score2 += 1,
            _ => { }
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