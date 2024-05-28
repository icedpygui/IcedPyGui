//!Styling
use iced::border::Radius;
use iced::{Background, Border, Color, Theme};
use iced::widget::container;

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