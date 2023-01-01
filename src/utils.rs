use crate::defines::fonts::FONT_PROGGY_CLEAN;
use barter_data::model::{Candle, DataKind, MarketEvent, PublicTrade};
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

pub fn split_ticker<'a>(input: &'a str) -> Result<(&'a str, &'a str), Error> {
    let parts: Vec<&str> = input.split("-").collect();
    if parts.len() != 2 {
        panic!("Invalid ticker");
    }
    Ok((parts[0], parts[1]))
}

pub fn get_trade(event: MarketEvent) -> Option<PublicTrade> {
    match event.kind {
        DataKind::Trade(trade) => Some(trade),
        _ => None,
    }
}

pub fn get_candle(event: MarketEvent) -> Option<Candle> {
    match event.kind {
        DataKind::Candle(candle) => Some(candle),
        _ => None,
    }
}
