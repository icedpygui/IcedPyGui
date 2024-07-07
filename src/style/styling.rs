//!Styling
use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::{Color, Theme};
use palette::{FromColor, Hsl, Mix};
use palette::rgb::Rgb;
use palette::color_difference::Wcag21RelativeContrast;
use pyo3::pyclass;


#[derive(Debug, Clone, Default)]
#[pyclass]
pub struct IpgPalette {
    pub id: usize,
    pub base: Option<Color>,
    pub strong: Option<Color>,
    pub weak: Option<Color>,
    pub strong_factor: f32,
    pub weak_factor: f32,
    pub bar: Option<Color>,
    pub border: Option<Color>,
    pub blur: Option<Color>,
    pub dot: Option<Color>,
    pub handle: Option<Color>,
    pub icon: Option<Color>,
    pub placeholder: Option<Color>,
    pub scroller: Option<Color>,
    pub scrollbar: Option<Color>,
    pub shadow: Option<Color>,
    pub text: Option<Color>,
}

impl IpgPalette {
    pub fn new(
        id: usize,
        base: Option<Color>,
        strong: Option<Color>,
        weak: Option<Color>,
        strong_factor: f32,
        weak_factor: f32,
        bar: Option<Color>,
        border: Option<Color>,
        blur: Option<Color>,
        dot: Option<Color>,
        handle: Option<Color>,
        icon: Option<Color>,
        placeholder: Option<Color>,
        scroller: Option<Color>,
        scrollbar: Option<Color>,
        shadow: Option<Color>,
        text: Option<Color>,
    ) -> Self {
        Self {
            id,
            base,
            strong,
            weak,
            strong_factor,
            weak_factor,
            bar,
            border,
            blur,
            dot,
            handle,
            icon,
            placeholder,
            scroller,
            scrollbar,
            shadow,
            text,
            }
    }
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgStyleStandard {
    Primary,
    Success,
    Danger,
    Text,
}

pub struct IpgStylingStandard {
    pub id: usize,
    pub standard: IpgStyleStandard
}

impl IpgStylingStandard {
    pub fn new(
        id: usize,
        standard: IpgStyleStandard,
    ) -> Self {
        Self {
            id,
            standard,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleBarColor {
    pub id: usize,
    pub color: Color,
    pub accent: f32,
}

impl StyleBarColor {
    pub fn new(
        id: usize,
        color: Color,
        accent: f32,
    ) -> Self {
        Self {
            id,
            color,
            accent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleBorder {
    pub id: usize,
    pub radius: Radius,
    pub width: f32,
    pub scroller_radius: Radius,
    pub scrollbar_radius: Radius,
}

impl StyleBorder {
    pub fn new(
        id: usize,
        radius: Radius,
        width: f32,
        scroller_radius: Radius,
        scrollbar_radius: Radius,
    ) -> Self {
        Self {
            id,
            radius,
            width,
            scroller_radius,
            scrollbar_radius,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleDotColor {
    pub id: usize,
    pub color: Color,
    pub accent: f32,
}

impl StyleDotColor {
    pub fn new(
        id: usize,
        color: Color,
        accent: f32,
    ) -> Self {
        Self {
            id,
            color,
            accent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleHandleColor {
    pub id: usize,
    pub color: Color,
    pub accent: f32,
}

impl StyleHandleColor {
    pub fn new(
        id: usize,
        color: Color,
        accent: f32,
    ) -> Self {
        Self {
            id,
            color,
            accent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleIconColor {
    pub id: usize,
    pub color: Color,
}

impl StyleIconColor {
    pub fn new(
        id: usize,
        color: Color,
    ) -> Self {
        Self {
            id,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleShadow {
    pub id: usize,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
}

impl StyleShadow {
    pub fn new(
        id: usize,
        offset_x: f32,
        offset_y: f32,
        blur_radius: f32,
    ) -> Self {
        Self {
            id,
            offset_x,
            offset_y,
            blur_radius,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleTextColor {
    pub id: usize,
    pub color: Color,
}

impl StyleTextColor {
    pub fn new(
        id: usize,
        color: Color,
    ) -> Self {
        Self {
            id,
            color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleFillMode {
    pub id: usize,
    pub full: Option<bool>,
    pub percent: Option<f32>,
    pub padded: Option<u16>,
    pub asymmetric_padding: Option<(u16, u16)>,
}

impl StyleFillMode {
    pub fn new(
        id: usize,
        full: Option<bool>,
        percent: Option<f32>,
        padded: Option<u16>,
        asymmetric_padding: Option<(u16, u16)>,
    ) -> Self {
        Self {
            id,
            full,
            percent,
            padded,
            asymmetric_padding,
        }
    }
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IpgColorPalette {
    /// The base background color.
    pub base: Pair,
    /// A weaker version of the base background color.
    pub weak: Pair,
    /// A stronger version of the base background color.
    pub strong: Pair,
}

impl IpgColorPalette {
    /// Generates a set of [`IpgColorPalette`] colors from the base and text colors.
    pub fn generate(base: Color, background: Color, text: Color, 
                mut strong_factor: Option<f32>, mut weak_factor: Option<f32>) -> Self {
        
        if weak_factor.is_none() {
            weak_factor = Some(0.4);
        }

        if strong_factor.is_none() {
            strong_factor = Some(0.1);
        }

        let weak = mix(base, background, weak_factor.unwrap()); 
        let strong = deviate(base, strong_factor.unwrap());

        Self {
            base: Pair::new(base, text),
            weak: Pair::new(weak, text),
            strong: Pair::new(strong, text),
        }
    }
}

pub fn mix(a: Color, b: Color, factor: f32) -> Color {
    let a_hsl = to_hsl(a);
    let b_hsl = to_hsl(b);

    let mixed = a_hsl.mix(b_hsl, factor);
    from_hsl(mixed)
}

fn deviate(color: Color, amount: f32) -> Color {
    if is_dark(color) {
        lighten(color, amount)
    } else {
        darken(color, amount)
    }
}

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

