#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use egui::special_emojis::GITHUB;

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
                    // put HTTP Post Method here
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
        initial_window_size: Some(egui::vec2(295.0, 270.0)),
        ..Default::default()
    };
    let result = eframe::run_native(
        "DBMInternRanking",
        native_options,
        Box::new(move |_ctx: &CreationContext<'_>| Box::new(app)),
    );

    result.unwrap_or_else(|e| eprintln!("Fehler beim Ausführen der Anwendung: {:?}", e));
}
