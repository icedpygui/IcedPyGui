use iced::{Background, Border, Color, Theme};
use iced::widget::container;

use crate::access_state;



pub fn get_container_styling(_theme: &Theme, id: usize) -> container::Style {
    dbg!("styling");
    let state = access_state();

    let background_opt = state.styling_background.get(&id);
    let border_opt = state.styling_border.get(&id);
    // let shadow_opt = state.styling_shadow.get(&id);

    let style = container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Color::TRANSPARENT,
            },
        ..Default::default()
        };

    if background_opt.is_some()  {
        let background = background_opt.unwrap();
        style.with_background(*background);
        }


    if border_opt.is_some() {
        let border = border_opt.unwrap();
        let color = border.color;
        let width = border.width;
        style.with_border(color, width);
    }
   
    drop(state);

    style

}