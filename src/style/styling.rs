//!Styling
use iced::theme::palette::Pair;
use iced::{Color, Theme};
use palette::{FromColor, Hsl};
use palette::rgb::Rgb;
use palette::color_difference::Wcag21RelativeContrast;
use pyo3::pyclass;


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgStyleStandard {
    Primary,
    Success,
    Danger,
    Text,
}

pub fn get_text_pair(text: Option<Color>, color: Color) -> Color {
    if text.is_some() {
        text.unwrap()
   } else {
        let mut t_color = Color::BLACK;
        if is_dark(color) {
            t_color = Color::WHITE;
        } 
        let pair = Pair::new(color, t_color);
        pair.text
   }
}

/// A set of background colors.
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct IpgColorPalette {
//     /// The base background color.
//     pub base: Pair,
//     /// A weaker version of the base background color.
//     pub weak: Pair,
//     /// A stronger version of the base background color.
//     pub strong: Pair,
// }

// impl IpgColorPalette {
//     /// Generates a set of [`IpgColorPalette`] colors from the base and text colors.
//     pub fn generate(base: Color, background: Color, text: Color, 
//                 mut strong_factor: Option<f32>, mut weak_factor: Option<f32>) -> Self {
        
//         if weak_factor.is_none() {
//             weak_factor = Some(0.4);
//         }

//         if strong_factor.is_none() {
//             strong_factor = Some(0.1);
//         }

//         let weak = mix(base, background, weak_factor.unwrap()); 
//         let strong = deviate(base, strong_factor.unwrap());

//         Self {
//             base: Pair::new(base, text),
//             weak: Pair::new(weak, text),
//             strong: Pair::new(strong, text),
//         }
//     }
// }

// pub fn mix(a: Color, b: Color, factor: f32) -> Color {
//     let a_hsl = to_hsl(a);
//     let b_hsl = to_hsl(b);

//     let mixed = a_hsl.mix(b_hsl, factor);
//     from_hsl(mixed)
// }

// fn deviate(color: Color, amount: f32) -> Color {
//     if is_dark(color) {
//         lighten(color, amount)
//     } else {
//         darken(color, amount)
//     }
// }

// pub fn weak(base: Color, background: Color, mut factor: Option<f32>) -> Color {

//     if factor.is_none() {
//         factor = Some(0.4);
//     }

//     let a_hsl = to_hsl(base);
//     let b_hsl = to_hsl(background);

//     let mixed = a_hsl.mix(b_hsl, factor.unwrap());
//     from_hsl(mixed)
// }

// pub fn strong(color: Color, mut amount: Option<f32>) -> Color {
//     if amount.is_none() {
//         amount = Some(0.1)
//     }
//     if is_dark(color) {
//         lighten(color, amount.unwrap())
//     } else {
//         darken(color, amount.unwrap())
//     }
// }

pub fn get_theme_color(wnd_theme: &Theme) -> Color {
    let palette = Theme::palette(wnd_theme);

    palette.background
}

pub fn is_dark(color: Color) -> bool {
    to_hsl(color).lightness < 0.6
}

pub fn darken(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lightness = if hsl.lightness - amount < 0.0 {
        0.0
    } else {
        hsl.lightness - amount
    };

    from_hsl(hsl)
}

pub fn lighten(color: Color, amount: f32) -> Color {
    let mut hsl = to_hsl(color);

    hsl.lightness = if hsl.lightness + amount > 1.0 {
        1.0
    } else {
        hsl.lightness + amount
    };

    from_hsl(hsl)
}

fn to_hsl(color: Color) -> Hsl {
    Hsl::from_color(Rgb::from(color))
}

fn from_hsl(hsl: Hsl) -> Color {
    Rgb::from_color(hsl).into()
}

pub fn readable(background: Color, text: Color) -> Color {
    if is_readable(background, text) {
        text
    } else {
        let white_contrast = relative_contrast(background, Color::WHITE);
        let black_contrast = relative_contrast(background, Color::BLACK);

        if white_contrast >= black_contrast {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}

fn is_readable(a: Color, b: Color) -> bool {
    let a_srgb = Rgb::from(a);
    let b_srgb = Rgb::from(b);

    a_srgb.has_enhanced_contrast_text(b_srgb)
}

fn relative_contrast(a: Color, b: Color) -> f32 {
    let a_srgb = Rgb::from(a);
    let b_srgb = Rgb::from(b);

    a_srgb.relative_contrast(b_srgb)
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgStyleParam {
    Background,
    BarColor,
    Border,
    DotColor,
    HandleColor,
    IconColor,
    Shadow,
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgStyleBackground {
    Accent,
    Color,
    Rgba,
}

