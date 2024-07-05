#![allow(dead_code)]

use iced::{Color, Theme};
use palette::rgb::Rgb;
use palette::{DarkenAssign, FromColor, LightenAssign, Mix, Okhsl, Srgb};
use rand::prelude::*;
use rand_chacha::ChaChaRng;



#[derive(Debug, Clone, Copy)]
pub struct IpgColorType {
    pub background: Color,
    pub text: Color,
    pub action: Color,
    pub accent: Color,
    pub alert: Color,
    pub error: Color,
    pub info: Color,
    pub success: Color,
}

impl Default for IpgColorType {
    fn default() -> IpgColorType {
        IpgColorType {
            background: hex_to_color("#2b292d").unwrap(),
            text: hex_to_color("#fecdb2").unwrap(),
            action: hex_to_color("#b1b695").unwrap(),
            accent: hex_to_color("#d1d1e0").unwrap(),
            alert: hex_to_color("#ffa07a").unwrap(),
            error: hex_to_color("#e06b75").unwrap(),
            info: hex_to_color("#f5d76e").unwrap(),
            success: hex_to_color("#b1b695").unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IpgColorAction {
    Base,
    Light,
    Lighter,
    Lightest,
    Dark,
    Darker,
    Darkest,
    LowAlpha,
    MedAlpha,
    HighAlpha,
}


pub fn get_alt_color(action: IpgColorAction, color: Color, is_dark: bool) -> Color {
    match action {
        IpgColorAction::Base => color,
        IpgColorAction::Light => lighten(color, 0.03),
        IpgColorAction::Lighter => lighten(color, 0.06),
        IpgColorAction::Lightest => lighten(color, 0.12),
        IpgColorAction::Dark => darken(color, 0.03),
        IpgColorAction::Darker => darken(color, 0.06),
        IpgColorAction::Darkest => darken(color, 0.12),
        IpgColorAction::LowAlpha => if is_dark {
                                        alpha(color, 0.4)
                                    } else {
                                        alpha(color, 0.8)
                                    },
        IpgColorAction::MedAlpha => if is_dark {
                                        alpha(color, 0.2)
                                    } else {
                                        alpha(color, 0.4)
                                    },
        IpgColorAction::HighAlpha => if is_dark {
                                        alpha(color, 0.1)
                                    } else {
                                        alpha(color, 0.3)
                                    },
        }
}


#[derive(Debug, Clone)]
pub enum Contrast {
    White,
    Black,
    Custom,
}

pub fn get_constrast_color(color: &Theme) -> Color {
    match color {
        Theme::Light => Color::BLACK,
        Theme::Dark => Color::WHITE,
        Theme::Dracula => Color::WHITE,
        Theme::Nord => Color::WHITE,
        Theme::SolarizedLight => Color::BLACK,
        Theme::SolarizedDark => Color::WHITE,
        Theme::GruvboxLight => Color::BLACK,
        Theme::GruvboxDark => Color::WHITE,
        Theme::CatppuccinLatte => Color::BLACK,
        Theme::CatppuccinFrappe => Color::WHITE,
        Theme::CatppuccinMacchiato => Color::WHITE,
        Theme::CatppuccinMocha => Color::WHITE,
        Theme::TokyoNight => Color::WHITE,
        Theme::TokyoNightStorm => Color::WHITE,
        Theme::TokyoNightLight => Color::WHITE,
        Theme::KanagawaWave => Color::WHITE,
        Theme::KanagawaDragon => Color::WHITE,
        Theme::KanagawaLotus => Color::BLACK,
        Theme::Moonfly => Color::WHITE,
        Theme::Nightfly => Color::WHITE,
        Theme::Oxocarbon => Color::WHITE,
        Theme::Ferra => Color::WHITE,
        Theme::Custom(_) => panic!("Constrast Color not found for a custom window color, supply your own background color"),
    }   
} 


fn hex_to_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 {
        let hash = &hex[0..1];
        let r = u8::from_str_radix(&hex[1..3], 16);
        let g = u8::from_str_radix(&hex[3..5], 16);
        let b = u8::from_str_radix(&hex[5..7], 16);

        return match (hash, r, g, b) {
            ("#", Ok(r), Ok(g), Ok(b)) => Some(Color {
                r: r as f32 / 255.0,
                g: g as f32 / 255.0,
                b: b as f32 / 255.0,
                a: 1.0,
            }),
            _ => None,
        };
    }

    None
}

/// Randomizes the hue value of an `iced::Color` based on a seed.
pub fn randomize_color(original_color: Color, seed: &str) -> Color {
    // Generate a 64-bit hash from the seed string
    let seed_hash = seahash::hash(seed.as_bytes());

    // Create a random number generator from the seed
    let mut rng = ChaChaRng::seed_from_u64(seed_hash);

    // Convert the original color to HSL
    let original_hsl = to_hsl(original_color);

    // Randomize the hue value using the random number generator
    let randomized_hue: f32 = rng.gen_range(0.0..=360.0);
    let randomized_hsl = Okhsl::new(
        randomized_hue,
        original_hsl.saturation,
        original_hsl.lightness,
    );

    // Convert the randomized HSL color back to Color
    from_hsl(randomized_hsl)
}

pub fn is_dark(color: Color) -> bool {
    to_hsl(color).lightness < 0.5
}

pub fn to_hsl(color: Color) -> Okhsl {
    let mut hsl = Okhsl::from_color(Rgb::from(color));
    if hsl.saturation.is_nan() {
        hsl.saturation = Okhsl::max_saturation();
    }

    hsl
}

pub fn from_hsl(hsl: Okhsl) -> Color {
    Srgb::from_color(hsl).into()
}

pub fn alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

pub fn mix(a: Color, b: Color, factor: f32) -> Color {
    let a_hsl = to_hsl(a);
    let b_hsl = to_hsl(b);

    let mixed = a_hsl.mix(b_hsl, factor);
    from_hsl(mixed)
}

pub fn lighten(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lighten_fixed_assign(amount);

    from_hsl(hsl)
}

pub fn darken(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.darken_fixed_assign(amount);

    from_hsl(hsl)
}
