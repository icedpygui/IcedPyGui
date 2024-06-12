
use iced::widget::rule::{self, FillMode, Style};
use iced::{Element, Length, Theme};
use iced::widget::{Container, Rule};
use crate::{access_state, app};

#[derive(Debug, Clone)]
pub struct IpgRule {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub thickness: u16,
    pub rule_type: String,
    pub style_background: Option<String>, 
    pub style_border: Option<String>,
    pub style_fill_mode: Option<String>,
}

impl IpgRule {
    pub fn new(
        id: usize, 
        width: Length, 
        height: Length,
        thickness: u16, 
        rule_type: String,
        style_background: Option<String>, 
        style_border: Option<String>,
        style_fill_mode: Option<String>,
        ) -> Self {
        Self {
            id,
            width,
            height,
            thickness,
            rule_type,
            style_background, 
            style_border,
            style_fill_mode,
        }
    }
}

// Looks reversed but not.  The only controllale parameter for horizontal
// is the thickness of the line which is height.  The opposite for vertical.
// To control the other dimension, need to put into a container.
pub fn construct_rule(rule: IpgRule) -> Element<'static, app::Message> {
    if rule.rule_type == "h".to_string() {
        return construct_horizontal(rule)
    } else {
        return construct_rule_vertical(rule)
    }

}

// The width or height parameters seems to have no effect so set to 0.
fn construct_horizontal(rule: IpgRule) -> Element<'static, app::Message>{

    let rule_h: Element<app::Message> = Rule::horizontal(rule.thickness)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    rule.style_background.clone(), 
                                                    rule.style_border.clone(), 
                                                    rule.style_fill_mode.clone(),
                                                    rule.thickness,
                                                    )  
                                                })
                                            .into();

    Container::new(rule_h).width(rule.width).into()

}

fn construct_rule_vertical(rule: IpgRule) -> Element<'static, app::Message> {

    let rule_v: Element<app::Message> = Rule::vertical(rule.thickness)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    rule.style_background.clone(), 
                                                    rule.style_border.clone(), 
                                                    rule.style_fill_mode.clone(),
                                                    rule.thickness,
                                                    )  
                                                })
                                                .into();

    Container::new(rule_v).height(rule.height).into()

}


fn get_styling(theme: &Theme,
                style_background: Option<String>, 
                style_border: Option<String>,
                style_fill_mode: Option<String>,
                thickness: u16,
                ) -> Style {

    let default_style = rule::default(theme);

    let state = access_state();

    let background_opt = if style_background.is_some() {
        state.styling_background.get(&style_background.unwrap())
    } else {
        None
    };

    let bg_color = if background_opt.is_some() {
        let color = background_opt.unwrap();
        color.color
    } else {
        default_style.color
    };

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let b_radius = if border_opt.is_some() {
        let border = border_opt.unwrap();
        border.radius
    } else {
        0.0.into()
    };

    let fill_mode_opt = if style_fill_mode.is_some() {
        state.styling_fill_mode.get(&style_fill_mode.unwrap())
    } else {
        None
    };

    let fill_mode: FillMode = if fill_mode_opt.is_some() {
        let f_mode = fill_mode_opt.unwrap();
        if f_mode.full.is_some() {
            FillMode::Full
        } else if f_mode.percent.is_some() {
            FillMode::Percent(f_mode.percent.unwrap())
        } else if f_mode.padded.is_some() {
            FillMode::Padded(f_mode.padded.unwrap())
        } else if f_mode.asymmetric_padding.is_some() {
            let a_padding = f_mode.asymmetric_padding.unwrap();
            FillMode::AsymmetricPadding(a_padding.0, a_padding.1)
        } else {
            FillMode::Full
        }
    } else {
        FillMode::Full
    };

    Style {
        color: bg_color,
        width: thickness,
        radius: b_radius,
        fill_mode,
    }

}
