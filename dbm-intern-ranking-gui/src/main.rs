#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use egui::special_emojis::GITHUB;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio;
#[derive(Deserialize, Debug)]
struct Player {
    id: u32,
    player_name: String,
}

#[derive(Deserialize, Debug)]
struct Fighter {
    id: u32,
    fighter_name: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Game {
    player_id: u32,
    fighter_id: u32,
    wins: u32,
    loses: u32
}

struct DBMInternRanking {
    player_id: u32,
    player_name: String,
    player_map: HashMap<u32, String>,

    fighter_id: u32,
    fighter_name: String,
    fighter_map: HashMap<u32, String>,

    wins: String,
    loses: String,
}

impl DBMInternRanking {
    fn new() -> Self {
        Self {
            player_id: 0,
            player_name: String::new(),
            player_map: HashMap::new(),

            fighter_id: 0,
            fighter_name: String::new(),
            fighter_map: HashMap::new(),

            wins: String::new(),
            loses: String::new(),
        }
    }

    async fn get_players(&mut self) -> Result<(), Error> {
        let response = reqwest::get("http://localhost:8080/ranking/player").await?;
        let players: Vec<Player> = response.json().await?;

        for player in players {
            self.player_map.insert(player.id, player.player_name);
        }

        Ok(())
    }

    async fn get_fighters(&mut self) -> Result<(), Error> {
        let response = reqwest::get("http://localhost:8080/ranking/fighter").await?;
        let fighters: Vec<Fighter> = response.json().await?;
        for fighter in fighters {
            self.fighter_map.insert(fighter.id, fighter.fighter_name);
        }

        Ok(())
    }
}

impl App for DBMInternRanking {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.2);
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DBMInternRanking");


            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::CollapsingHeader::new("Open me!")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("This is a ui!");
                    ui.label("The default font supports all latin and cyrillic characters (ИÅđ…), common math symbols (∫√∞²⅓…), and many emojis (💓🌟🖩…).")
                    .on_hover_text("Why are you hovering me? ;(");
                });
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Player Name");
                        egui::ComboBox::from_id_source("player_dropdown")
                            .selected_text(self.player_name.clone())
                            .show_ui(ui, |ui| {
                                for (player_id, player_name) in &self.player_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.player_name,
                                            player_name.clone(),
                                            player_name,
                                        )
                                        .clicked()
                                    {
                                        self.player_id = *player_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("Fighter Name");
                        egui::ComboBox::from_id_source("fighter_dropdown")
                            .selected_text(self.fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.fighter_id = *fighter_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("Wins");
                        ui.add(egui::TextEdit::singleline(&mut self.wins));
                        ui.end_row();

                        ui.label("Loses");
                        ui.add(egui::TextEdit::singleline(&mut self.loses));
                        ui.end_row();
                    });
                ui.add_space(10.0);

                if ui.button("Send").clicked() {
                    let player_id = self.player_id;
                    let fighter_id = self.fighter_id;
                    let wins = self.wins.parse().unwrap_or(0);
                    let loses = self.loses.parse().unwrap_or(0);
                
                    let _ = tokio::spawn(async move {
                        let game = Game {
                            player_id,
                            fighter_id,
                            wins,
                            loses,
                        };
                
                        let client = reqwest::Client::new();
                        let response = client.post("http://localhost:8080/ranking/game")
                            .json(&game)
                            .send()
                            .await;
                
                        match response {
                            Ok(response) => {
                                if response.status().is_success() {
                                    println!("Game posted successfully");
                                } else {
                                    eprintln!("Failed to post game: {}", response.status());
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to post game: {}", e);
                            }
                        }
                    });
                    println!(
                        "player_id: {}\nplayer_name: {}\nfighter_id: {}\nfighter_name: {}\nwins: {}\nloses: {}\n",
                        self.player_id,
                        self.player_name,
                        self.fighter_id,
                        self.fighter_name,
                        self.wins,
                        self.loses,
                    );
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
        ..Default::default()
    };

    if let Err(e) = app.get_players().await {
        eprintln!("Failed to get players: {}", e);
    }

    if let Err(e) = app.get_fighters().await {
        eprintln!("Failed to get fighters: {}", e);
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
