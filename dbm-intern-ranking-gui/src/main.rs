#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use egui::special_emojis::GITHUB;
use reqwest::blocking::Client;

struct DBMInternRanking {
    playername: String,
    games_played: String,
    wins: String,
    loses: String,
    charakter: String,
    dark_mode: bool,
}

impl DBMInternRanking {
    fn new() -> Self {
        Self {
            playername: String::new(),
            games_played: String::new(),
            wins: String::new(),
            loses: String::new(),
            charakter: String::new(),
            dark_mode: true,
        }
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
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Player Name");
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
                    let client = Client::new();
                    let playername_url = client
                        .post(&format!(
                            "http://212.132.108.197:8000/ranking/playername?msg={}",
                            self.playername
                        ))
                        .send();
                    let games_url = client
                        .post(&format!(
                            "http://212.132.108.197:8000/ranking/games-played?msg={}",
                            self.games_played
                        ))
                        .send();
                    let wins = client
                        .post(&format!(
                            "http://212.132.108.197:8000/ranking/wins?msg= {}",
                            self.wins
                        ))
                        .send();
                    let loses_url = client
                        .post(&format!(
                            "http://212.132.108.197:8000/ranking/loses?msg= {}",
                            self.loses
                        ))
                        .send();
                    let charakter_url = client
                        .post(&format!(
                            "http://212.132.108.197:8000/ranking/chars?msg= {}",
                            self.charakter
                        ))
                        .send();
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

fn main() {
    let app = DBMInternRanking::new();
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    let result = eframe::run_native(
        "DBMInternRanking",
        native_options,
        Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)),
    );

    result.unwrap_or_else(|e| eprintln!("Fehler beim Ausführen der Anwendung: {:?}", e));
}
