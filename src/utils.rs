use crate::defines::fonts::FONT_PROGGY_CLEAN;
use eframe::{
    egui,
    egui::{FontData, FontDefinitions},
};

pub fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "ProggyClean".to_owned(),
        FontData::from_static(&FONT_PROGGY_CLEAN),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "ProggyClean".to_owned());
    ctx.set_fonts(fonts);
}
