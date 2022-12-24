use chrono::{DateTime, Utc};
use eframe::egui;
use egui::Ui;

pub fn show(ui: &mut Ui) {
    ui.separator();
    use egui_extras::{Column, TableBuilder};

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::auto())
        .column(Column::auto())
        //.column(Column::initial(50.0).range(40.0..=300.0).resizable(true))
        // .column(
        //     Column::initial(100.0)
        //         .at_least(40.0)
        //         .resizable(true)
        //         .clip(true),
        // )
        .column(Column::remainder())
        .min_scrolled_height(0.0);
    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Exchange");
            });
            header.col(|ui| {
                ui.strong("Price");
            });
            header.col(|ui| {
                ui.strong("Size");
            });
            header.col(|ui| {
                ui.strong("Time");
            });
        })
        .body(|mut body| {
            for row_index in 0..255 {
                body.row(18.0, |mut row| {
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = "coinbase pro";
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                ..Default::default()
                            },
                        );

                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = "45,325.00";
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: if row_index % 2 == 0 {
                                    egui::Color32::RED
                                } else {
                                    egui::Color32::GREEN
                                },
                                // background: egui::Color32::RED,
                                // background: global::COLOR_RED_TRANSPARENT,
                                ..Default::default()
                            },
                        );

                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        let mut layout_job = egui::text::LayoutJob::default();
                        let text = "0.5";
                        layout_job.append(
                            &text,
                            0.0,
                            egui::text::TextFormat {
                                font_id: egui::FontId::monospace(15.0),
                                color: egui::Color32::WHITE,
                                // background: egui::Color32::RED,
                                background: egui::Color32::from_rgba_unmultiplied(
                                    255, 0, 0, row_index,
                                ),
                                ..Default::default()
                            },
                        );
                        ui.label(layout_job);
                    });
                    row.col(|ui| {
                        // ui.label(RichText::new("ProggyClean").)
                        let now: DateTime<Utc> = Utc::now();
                        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
                        ui.monospace(timestamp);
                    });
                });
            }
        });
}
