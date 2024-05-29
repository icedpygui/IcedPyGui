//!Styling
use iced::border::Radius;
use iced::{Background, Border, Color, Theme};
use iced::widget::container;
use palette::{FromColor, Hsl};
use palette::rgb::Rgb;

use crate::access_state;



pub fn get_container_styling(_theme: &Theme, id: usize) -> container::Style {
    
    let state = access_state();

    let background_opt = state.styling_background.get(&id);
    let border_opt = state.styling_border.get(&id);
    let shadow_opt = state.styling_shadow.get(&id);
    let text_color_opt = state.styling_text_color.get(&id);

    let background = match background_opt {
        Some(bg) => *bg,
        None => Background::Color(Color::TRANSPARENT),
    };

    let border = match border_opt {
        Some(bd) => *bd,
        None => Border{color: Color::TRANSPARENT, radius: Radius::from([5.0; 4]), width: 1.0},
    };

    let shadow = match shadow_opt {
        Some(sh) => *sh,
        None => Default::default(),
    };

    let text_color = match text_color_opt {
        Some(tc) => Some(*tc),
        None => None,
    };


    let style = container::Style {
        background: Some(background),
        border,
        shadow,
        text_color,
        };

    style

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
