#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use egui::{special_emojis::GITHUB, RichText, ViewportBuilder};
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
    first_fighter_id: u32,
    second_fighter_id: u32,
    third_fighter_id: u32,
    fourth_fighter_id: u32,
    fifth_fighter_id: u32,
    wins: u32,
    loses: u32
}

struct DBMInternRanking {
    player_id: u32,
    player_name: String,
    player_map: HashMap<u32, String>,

    first_fighter_id: u32,
    first_fighter_name: String,

    second_fighter_id: u32,
    second_fighter_name: String,

    third_fighter_id: u32,
    third_fighter_name: String,

    fourth_fighter_id: u32,
    fourth_fighter_name: String,

    fifth_fighter_id: u32,
    fifth_fighter_name: String,

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

            first_fighter_id: 0,
            first_fighter_name: String::new(),

            second_fighter_id: 0,
            second_fighter_name: String::new(),

            third_fighter_id: 0,
            third_fighter_name: String::new(),

            fourth_fighter_id: 0,
            fourth_fighter_name: String::new(),

            fifth_fighter_id: 0,
            fifth_fighter_name: String::new(),

            fighter_map: HashMap::new(),

            wins: String::new(),
            loses: String::new(),
        }
    }

    async fn get_players(&mut self) -> Result<(), Error> {
        let response = reqwest::get("someurl").await?;
        let players: Vec<Player> = response.json().await?;

        for player in players {
            self.player_map.insert(player.id, player.player_name);
        }

        Ok(())
    }

    async fn get_fighters(&mut self) -> Result<(), Error> {
        let response = reqwest::get("someurl").await?;
        let fighters: Vec<Fighter> = response.json().await?;
        for fighter in fighters {
            self.fighter_map.insert(fighter.id, fighter.fighter_name);
        }

        Ok(())
    }
}

impl App for DBMInternRanking {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.4);
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DBMInternRanking");


            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Player *");
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

                        ui.label("1. Fighter *");
                        egui::ComboBox::from_id_source("first_fighter_dropdown")
                            .selected_text(self.first_fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.first_fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.first_fighter_id = *fighter_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("2. Fighter");
                        egui::ComboBox::from_id_source("second_fighter_dropdown")
                            .selected_text(self.second_fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.second_fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.second_fighter_id = *fighter_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("3. Fighter");
                        egui::ComboBox::from_id_source("third_fighter_dropdown")
                            .selected_text(self.third_fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.third_fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.third_fighter_id = *fighter_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("4. Fighter");
                        egui::ComboBox::from_id_source("fourth_fighter_dropdown")
                            .selected_text(self.fourth_fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.fourth_fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.fourth_fighter_id = *fighter_id;
                                    }
                                }
                            });
                        ui.end_row();

                        ui.label("5. Fighter");
                        egui::ComboBox::from_id_source("fifth_fighter_dropdown")
                            .selected_text(self.fifth_fighter_name.clone())
                            .show_ui(ui, |ui| {
                                for (fighter_id, fighter_name) in &self.fighter_map {
                                    if ui
                                        .selectable_value(
                                            &mut self.fifth_fighter_name,
                                            fighter_name.clone(),
                                            fighter_name,
                                        )
                                        .clicked()
                                    {
                                        self.fifth_fighter_id = *fighter_id;
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
                ui.add_space(5.0);
                ui.label(RichText::new("* required").small().weak());
                ui.add_space(10.0);

                if ui.button(RichText::new("    Send    ").heading()).clicked() {
                    let player_id = self.player_id;
                    let first_fighter_id = self.first_fighter_id;
                    let second_fighter_id = self.second_fighter_id;
                    let third_fighter_id = self.third_fighter_id;
                    let fourth_fighter_id = self.fourth_fighter_id;
                    let fifth_fighter_id = self.fifth_fighter_id;
                    let wins = self.wins.parse().unwrap_or(0);
                    let loses = self.loses.parse().unwrap_or(0);
                
                    let _ = tokio::spawn(async move {
                        let game = Game {
                            player_id,
                            first_fighter_id,
                            second_fighter_id,
                            third_fighter_id,
                            fourth_fighter_id,
                            fifth_fighter_id,
                            wins,
                            loses
                        
                        };
                
                        let client = reqwest::Client::new();
                        let response = client.post("someurl")
                            .json(&game)
                            .send()
                            .await;
                
                        match response {
                            Ok(response) => {
                                if response.status().is_success() {
                                    println!("Game posted successfully");
                                } else {
                                    eprintln!("Could not post game: {}", response.status());
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to post game: {}", e);
                            }
                        }
                    });
                    
                    println!(
                        "player_id: {}\nplayer_name: {}\nfirst_fighter_id: {}\nfirst_fighter_name: {}\nsecond_fighter_id: {}\nsecond_fighter_name: {}\nthird_fighter_id: {}\nthird_fighter_name: {}\nfourth_fighter_id: {}\nfourth_fighter_name: {}\nfifth_fighter_id: {}\nfifth_fighter_name: {}\nwins: {}\nloses: {}\n",
                        self.player_id,
                        self.player_name,
                        self.first_fighter_id,
                        self.first_fighter_name,
                        self.second_fighter_id,
                        self.second_fighter_name,
                        self.third_fighter_id,
                        self.third_fighter_name,
                        self.fourth_fighter_id,
                        self.fourth_fighter_name,
                        self.fifth_fighter_id,
                        self.fifth_fighter_name,
                        self.wins,
                        self.loses,
                    );
                }
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
        viewport: ViewportBuilder {
            inner_size: Some(egui::Vec2::new(350.0, 430.0)),
            min_inner_size: Some(egui::Vec2::new(349.0, 429.0)),
            ..Default::default()
        },
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

