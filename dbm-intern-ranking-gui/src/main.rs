#![windows_subsystem = "windows"]

use std::collections::HashMap;
use tokio;
use eframe::{egui, App, CreationContext, Frame};
use egui::special_emojis::GITHUB;
use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct Player {
    id: u32,
    playername: String,
}

struct DBMInternRanking {
    id: u32,
    playername: String,
    games_played: String,
    wins: String,
    loses: String,
    charakter: String,
    dark_mode: bool,
    player_map: HashMap<String, u32>,
}

impl DBMInternRanking {
    fn new() -> Self {
        Self {
            id: 0,
            playername: String::new(),
            games_played: String::new(),
            wins: String::new(),
            loses: String::new(),
            charakter: String::new(),
            dark_mode: true,
            player_map: HashMap::new(),
        }
    }
    async fn get_players(&mut self) -> Result<(), Error> {
        let response = reqwest::get("http://localhost:8080/player").await?;
        let players: Vec<Player> = response.json().await?;

        for player in players {
            self.player_map.insert(player.playername, player.id);
        }

        Ok(())
    }
}

impl App for DBMInternRanking {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.2);

        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DBMInternRanking");

                ui.with_layout(
                    egui::Layout::from_main_dir_and_cross_align(
                        egui::Direction::RightToLeft,
                        egui::Align::RIGHT,
                    ),
                    |ui| {
                        let button_text = if self.dark_mode { "🌙" } else { "🌞" };
                        if ui.button(button_text).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }
                    },
                );
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Player Name");
                        egui::ComboBox::from_id_source("player_dropdown")
                            .selected_text(self.playername.clone())
                            .show_ui(ui, |ui| {
                                for (playername, id) in &self.player_map {
                                    if ui.selectable_value(&mut self.playername, playername.clone(), playername).clicked() {
                                        self.id = *id;
                                    }
                                }
                            });
                        ui.end_row();
                        
                        ui.label("Player Name OLD");
                        ui.add(egui::TextEdit::singleline(&mut self.playername));
                        ui.end_row();

                        ui.label("Games played");
                        ui.add(egui::TextEdit::singleline(&mut self.games_played));
                        ui.end_row();

                        ui.label("Wins");
                        ui.add(egui::TextEdit::singleline(&mut self.wins));
                        ui.end_row();

                        ui.label("Loses");
                        ui.add(egui::TextEdit::singleline(&mut self.loses));
                        ui.end_row();

                        ui.label("Charakter");
                        ui.add(egui::TextEdit::singleline(&mut self.charakter));
                        ui.end_row();
                    });
                ui.add_space(10.0);

                if ui.button("Send").clicked() {
                    println!("{}\n{}\n{}\n{}\n{}\n{}", self.id, self.playername, self.games_played, self.wins, self.loses, self.charakter);
                }
                ui.add_space(10.0);
            });

            egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.hyperlink_to(
                            format!("{GITHUB} @JasonClarkAltmann"),
                            "https://github.com/JasonClarkAltmann",
                        );

                        ui.hyperlink_to(
                            format!("{GITHUB} @KyokoSpl"),
                            "https://github.com/KyokoSpl",
                        );
                    });

                    ui.with_layout(
                        egui::Layout::from_main_dir_and_cross_align(
                            egui::Direction::RightToLeft,
                            egui::Align::RIGHT,
                        ),
                        |ui| {
                            ui.hyperlink_to(
                                format!("{GITHUB} Source Code"),
                                "https://github.com/KyokoSpl/dbm-intern-ranking/",
                            );
                        },
                    );
                });
            });
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut app = DBMInternRanking::new();
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(310.0, 285.0)),
        ..Default::default()
    };

    if let Err(e) = app.get_players().await {
        eprintln!("Failed to get players: {}", e);
    }

    let result = eframe::run_native(
        "DBMInternRanking",
        native_options,
        Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)),
    );

    if let Err(e) = result {
        eprintln!("An error occurred: {}", e);
    }

    Ok(())
}
