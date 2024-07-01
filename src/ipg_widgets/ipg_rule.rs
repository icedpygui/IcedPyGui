
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
    pub style_color: Option<String>, 
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
        style_color: Option<String>, 
        style_border: Option<String>,
        style_fill_mode: Option<String>,
        ) -> Self {
        Self {
            id,
            width,
            height,
            thickness,
            rule_type,
            style_color, 
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

    let rule_h: Element<app::Message> = Rule::horizontal(1)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    rule.style_color.clone(), 
                                                    rule.style_border.clone(), 
                                                    rule.style_fill_mode.clone(),
                                                    rule.thickness,
                                                    )  
                                                })
                                            .into();

    Container::new(rule_h).width(rule.width).into()

}

fn construct_rule_vertical(rule: IpgRule) -> Element<'static, app::Message> {

    let rule_v: Element<app::Message> = Rule::vertical(1)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    rule.style_color.clone(), 
                                                    rule.style_border.clone(), 
                                                    rule.style_fill_mode.clone(),
                                                    rule.thickness,
                                                    )  
                                                })
                                                .into();

    Container::new(rule_v).height(rule.height).into()

}


fn get_styling(theme: &Theme,
                style_color: Option<String>, 
                style_border: Option<String>,
                style_fill_mode: Option<String>,
                thickness: u16,
                ) -> Style {

    let mut base_style = rule::default(theme);

    if style_color.is_none() && style_border.is_none() && style_fill_mode.is_none() {
        return  base_style
    }

    let state = access_state();

    let color_palette_opt = if style_color.is_some() {
        state.styling_color.get(&style_color.unwrap())
    } else {
        None
    };

    if color_palette_opt.is_some() {
        let color_palette = color_palette_opt.unwrap();
        if color_palette.base.is_some() {
            base_style.color = color_palette.base.unwrap();
        }
    }

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    if border_opt.is_some() {
        let border = border_opt.unwrap();
        base_style.radius = border.radius; 
    }

    base_style.width = thickness;

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

    base_style.fill_mode = fill_mode;

    base_style

}
