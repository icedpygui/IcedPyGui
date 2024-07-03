//!Styling
use iced::border::Radius;
use iced::{Background, Color, Theme};
use iced::widget::container;
use palette::{FromColor, Hsl};
use palette::rgb::Rgb;
use pyo3::pyclass;


#[derive(Debug, Clone, Default)]
#[pyclass]
pub struct IpgPalette {
    pub id: usize,
    pub base: Option<Color>,
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

// pub fn style_background_update(bkg: &mut StyleBackground, item: PyObject, value: PyObject) {
//     let update = try_extract_bkg_update(item);

//     match update {
//         IpgStyleBackgroundParam::Accent => {
//             let val = try_extract_f64(value);
//             bkg.accent_amount = val as f32;
//         },
//         IpgStyleBackgroundParam::Color => {
//             let val = try_extract_color(value);
//             bkg.color = val;
//         },
//         IpgStyleBackgroundParam::Rgba => {
//             let val = try_extract_vec_f32(value);
//             let rgba: [f32; 4] = [val[0], val[1], val[2], val[3]];
//             let color = get_color(Some(rgba), None, 1.0, false);
//             bkg.color = color;
//         }
//     }
// }

// pub fn try_extract_bkg_update(update_obj: PyObject) -> IpgStyleBackgroundParam {

//     Python::with_gil(|py| {
//         let res = update_obj.extract::<IpgStyleBackgroundParam>(py);
//         match res {
//             Ok(update) => update,
//             Err(_) => panic!("Style Background update extraction failed"),
//         }
//     })
// }

// pub fn try_extract_color(color: PyObject) -> Color {

//     Python::with_gil(|py| {
//         let res = color.extract::<IpgColor>(py);
//         match res {
//             Ok(col) => get_color(None, Some(col), 1.0, false),
//             Err(_) => panic!("Style Background color extraction failed"),
//         }
//     })
// }
