#![windows_subsystem = "windows"]

use eframe::{egui, App, CreationContext, Frame};
use egui::special_emojis::GITHUB;

struct DBMInternRanking {
    text1: String,
    text2: String,
    text3: String,
    text4: String,
    text5: String,
    dark_mode: bool,
}

impl DBMInternRanking {
    fn new() -> Self {
        Self {
            text1: String::new(),
            text2: String::new(),
            text3: String::new(),
            text4: String::new(),
            text5: String::new(),
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
                        ui.label("Text1");
                        ui.add(egui::TextEdit::singleline(&mut self.text1));
                        ui.end_row();

                        ui.label("Text2");
                        ui.add(egui::TextEdit::singleline(&mut self.text2));
                        ui.end_row();

                        ui.label("Text3");
                        ui.add(egui::TextEdit::singleline(&mut self.text3));
                        ui.end_row();

                        ui.label("Text4");
                        ui.add(egui::TextEdit::singleline(&mut self.text4));
                        ui.end_row();

                        ui.label("Text5");
                        ui.add(egui::TextEdit::singleline(&mut self.text5));
                        ui.end_row();
                    });
                ui.add_space(10.0);

                if ui.button("Paste").clicked() {
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
