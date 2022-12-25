// Store configuration stuff here
pub const APP_NAME: &str = "enigma machine";
// pub const APP_CONFIG_NAME: &str = "tiny_pomodoro.yaml";

pub mod fonts {
    use include_flate::flate;
    flate!(pub static FONT_PROGGY_CLEAN: [u8] from "resources/ProggyClean.ttf");
}
