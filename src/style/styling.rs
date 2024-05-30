//!Styling
use iced::border::Radius;
use iced::{Background, Border, Color, Theme};
use iced::widget::container;
use palette::{FromColor, Hsl};
use palette::rgb::Rgb;


#[derive(Debug, Clone)]
pub struct StyleBackground {
    pub id: usize,
    pub color: Color,
    pub hover_factor: f32,
}

impl StyleBackground {
    pub fn new(
        id: usize,
        color: Color,
        hover_factor: f32,
    ) -> Self {
        Self {
            id,
            color,
            hover_factor,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleBorder {
    pub id: usize,
    pub color: Color,
    pub radius: Radius,
    pub width: f32,
}

impl StyleBorder {
    pub fn new(
        id: usize,
        color: Color,
        radius: Radius,
        width: f32,
    ) -> Self {
        Self {
            id,
            color,
            radius,
            width,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleShadow {
    pub id: usize,
    pub color: Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_radius: f32,
}

impl StyleShadow {
    pub fn new(
        id: usize,
        color: Color,
        offset_x: f32,
        offset_y: f32,
        blur_radius: f32,
    ) -> Self {
        Self {
            id,
            color,
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




pub fn date_picker_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0.7, 0.5, 0.6, 1.0))),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Color::TRANSPARENT,
        },
        ..Default::default()
    }
}

use crate::ipg_widgets::ipg_table::TableRowHighLight;
pub fn table_row_theme(theme: &Theme, idx: usize, amount: f32, 
                        highlighter: Option<TableRowHighLight>) -> container::Style {

    let mut background = get_theme_color(theme);

    if idx % 2 == 0 {
        background = match highlighter {
                Some(hl) => 
                    match hl {
                        TableRowHighLight::Darker => darken(background, amount),
                        TableRowHighLight::Lighter => lighten(background, amount),
                        },
                None => background,
            }
    }; 
    
    container::Style {
        background: Some(Background::Color(background)),
        ..Default::default()
    }
}

fn get_theme_color(wnd_theme: &Theme) -> Color {
    let palette = Theme::palette(wnd_theme);

    palette.background
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
