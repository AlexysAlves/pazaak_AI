use eframe::{egui, Frame};
use core::{GameState, SideCard, Action};
use agents::{Agent, RandomAgent};

struct PazaakApp {
    state: GameState,
    human_is_player1: bool,
    agent: RandomAgent,
}

impl Default for PazaakApp {
    fn default() -> Self {
        // create pools and initial state 
        let pool1 = SideCard::random_pool(10);
        let pool2 = SideCard::random_pool(10);
        let state = GameState::new(pool1, pool2);
        Self {
            state,
            human_is_player1: true,
            agent: RandomAgent::new(),
        }
    }
}

impl eframe::App for PazaakApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pazaak");

            // Show scores and whose turn
            ui.horizontal(|ui| {
                ui.label(format!("Player score: {}", self.state.player.score));
                ui.label(format!("Opponent score: {}", self.state.opponent.score));
                ui.label(format!("Turn: {}", if self.state.player_turn { "Player" } else { "Opponent" }));
            });

            ui.separator();

            // Show human hand 
            let (hand, _pool_len) = if self.human_is_player1 {
                (&self.state.player.side_hand, self.state.player.side_pool.len())
            } 
            else {
                (&self.state.opponent.side_hand, self.state.opponent.side_pool.len())
            };

            ui.label("Your hand:");
            for (i, card) in hand.iter().enumerate() {
                let text = match card {
                    core::SideCard::Simple(v) => format!("{}", v),
                    core::SideCard::Flip(m) => format!("Flip({})", m),
                };
                ui.label(format!("{}: {}", i, text));
            }

            ui.separator();

            // actions
            let legal = self.state.legal_actions();
            ui.label("Actions:");
            for a in legal.iter() {
                let label = match a {
                    Action::Stand => "Stand".to_string(),
                    Action::EndTurn => "EndTurn".to_string(),
                    Action::PlaySide(i, opt) => {
                        match opt {
                            Some(true) => format!("PlaySide {} (+)", i),
                            Some(false) => format!("PlaySide {} (-)", i),
                            None => format!("PlaySide {} (?)", i),
                        }
                    }
                };
                if ui.button(label).clicked() {
                    // human plays action
                    let is_human_turn = (self.human_is_player1 && self.state.player_turn)
                        || (!self.human_is_player1 && !self.state.player_turn);
                    if is_human_turn {
                        self.state.apply_action(*a);
                    }
                }
            }

            ui.separator();

            let agent_turn = (!self.human_is_player1 && self.state.player_turn) || (self.human_is_player1 && !self.state.player_turn);
            if agent_turn {
                if ui.button("Agent move").clicked() {
                    // get action and apply
                    let agent_ref: &mut dyn Agent = &mut self.agent;
                    let action = agent_ref.select_action(&self.state);
                    self.state.apply_action(action);
                }
            }

            // show winner if round over
            if self.state.is_round_over() {
                if let Some(w) = self.state.round_winner() {
                    ui.label(format!("Round winner: {}", w));
                } 
                else {
                    ui.label("Round is a tie.");
                }
                if ui.button("Reset round").clicked() {
                    let pool1 = SideCard::random_pool(10);
                    let pool2 = SideCard::random_pool(10);
                    self.state = GameState::new(pool1, pool2);
                }
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pazaak",
        native_options,
        Box::new(|_cc| Box::new(PazaakApp::default())),
    ).expect("failed to run eframe");
}