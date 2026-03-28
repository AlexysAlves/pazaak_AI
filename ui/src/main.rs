use eframe::{egui, Frame};
use core::{GameState, SideCard, Action};
use agents::{Agent, RandomAgent};

struct PazaakApp {
    state: GameState,
    human_is_player1: bool,
    agent: RandomAgent,
}

fn format_side_card(card: &SideCard) -> String {
    match card {
        SideCard::Simple(v) => format!("{}", v),
        SideCard::Flip(v) => format!("Flip({})", v),
    }
}

fn format_card_event(event: &core::CardEvent) -> String {
    match event {
        core::CardEvent::MainDeck(v) => format!("Main deck: {}", v),
        core::CardEvent::SideDeck(card) => format!("Side card: {}", format_side_card(card)),
    }
}

fn show_player_panel(ui: &mut egui::Ui, title: &str, player: &core::PlayerState) {
    ui.push_id(title, |ui| {
        ui.group(|ui| {
            ui.heading(title);
            ui.label(format!("Score: {}", player.score));
            ui.label(format!("Stood: {}", if player.stood { "yes" } else { "no" }));

            ui.collapsing("Initial side deck (10)", |ui| {
                for (i, card) in player.side_deck_all.iter().enumerate() {
                    ui.label(format!("{}: {}", i + 1, format_side_card(card)));
                }
            });

            ui.collapsing("Current side hand", |ui| {
                for (i, card) in player.side_hand.iter().enumerate() {
                    ui.label(format!("{}: {}", i + 1, format_side_card(card)));
                }
            });

            ui.collapsing("Cards played", |ui| {
                if player.played_cards.is_empty() {
                    ui.label("None");
                } else {
                    for (i, event) in player.played_cards.iter().enumerate() {
                        ui.label(format!("{}: {}", i + 1, format_card_event(event)));
                    }
                }
            });
        });
    });
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

            ui.horizontal(|ui| {
                show_player_panel(ui, "Player", &self.state.player);
                ui.separator();
                show_player_panel(ui, "Opponent", &self.state.opponent);
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