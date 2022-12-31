use crate::defines::fonts::FONT_PROGGY_CLEAN;
use barter_data::model::{DataKind, MarketEvent, PublicTrade};
use barter_integration::model::Side;
use eframe::egui::{self, FontData, FontDefinitions};
use std::io::Error;

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

pub fn split_ticker(input: String) -> Result<(String, String), Error> {
    let parts: Vec<&str> = input.as_str().split("-").collect();
    if parts.len() != 2 {
        panic!("Invalid ticker");
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub fn get_trade(event: MarketEvent) -> PublicTrade {
    match event.kind {
        DataKind::Trade(trade) => trade,
        _ => PublicTrade {
            id: "123".to_string(),
            price: 123.,
            quantity: 123.,
            side: Side::Buy,
        },
    }
}
