//! ipg_rule
use iced::widget::rule::{self, FillMode, Style};
use iced::{Color, Element, Length, Theme};
use iced::widget::{Container, Rule};
use pyo3::{pyclass, PyObject, Python};
use crate::app;
use crate::graphics::colors::get_color;

use super::helpers::{get_radius, try_extract_f64, try_extract_ipg_color, 
    try_extract_rgba_color, try_extract_u16, try_extract_vec_f32, 
    try_extract_vec_u16};
use super::ipg_enums::IpgWidgets;

#[derive(Debug, Clone)]
pub struct IpgRule {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub thickness: u16,
    pub rule_type: String,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl IpgRule {
    pub fn new(
        id: usize, 
        width: Length, 
        height: Length,
        thickness: u16, 
        rule_type: String,
        style_id: Option<usize>,
        show: bool,
        ) -> Self {
        Self {
            id,
            width,
            height,
            thickness,
            rule_type,
            style_id,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgRuleStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub fillmode_percent: Option<f32>,
    pub fillmode_padded: Option<u16>,
    pub fillmode_asymmetric_padding: Option<Vec<u16>>,
}

impl IpgRuleStyle {
    pub fn new(
        id: usize,
        color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        fillmode_percent: Option<f32>,
        fillmode_padded: Option<u16>,
        fillmode_asymmetric_padding: Option<Vec<u16>>,
    ) -> Self {
        Self {
            id,
            color,
            border_radius,
            fillmode_percent,
            fillmode_padded,
            fillmode_asymmetric_padding,
        }
    }
}

// Looks reversed but not.  The only controllable parameter for horizontal
// is the thickness of the line which is height.  The opposite for vertical.
// To control the other dimension, need to put into a container.
pub fn construct_rule(rule: IpgRule, 
                        style_opt: Option<IpgWidgets>) 
                        -> Option<Element<'static, app::Message>> {

    if !rule.show {
        return None
    }

    if rule.rule_type == *"h" {
        Some(construct_horizontal(rule, style_opt))
    } else {
        Some(construct_rule_vertical(rule, style_opt))
    }
}

// The width or height parameters seems to have no effect so set to 0.
pub fn construct_horizontal(rule: IpgRule, 
                            style_opt: Option<IpgWidgets>) 
                            -> Element<'static, app::Message>{

    let style = get_rule_style(style_opt);

    let rule_h: Element<app::Message> = Rule::horizontal(1)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    style.clone(),
                                                    rule.thickness, 
                                                    )  
                                                })
                                            .into();

    Container::new(rule_h).width(rule.width).into()

}

fn construct_rule_vertical(rule: IpgRule, 
                            style_opt: Option<IpgWidgets>) 
                            -> Element<'static, app::Message> {

    let style = get_rule_style(style_opt);

    let rule_v: Element<app::Message> = Rule::vertical(1)
                                            .style(move|theme: &Theme| {   
                                                get_styling(theme,
                                                    style.clone(), 
                                                    rule.thickness,
                                                    )  
                                                })
                                                .into();

    Container::new(rule_v).height(rule.height).into()

}

fn get_rule_style(style: Option<IpgWidgets>) -> Option<IpgRuleStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgRuleStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}


fn get_styling(theme: &Theme,
                style_opt: Option<IpgRuleStyle>, 
                thickness: u16,
                ) -> Style {

    let mut base_style = rule::default(theme);
    base_style.width = thickness;

    if style_opt.is_none() {
        return  base_style
    }

    let style = style_opt.unwrap();
    
    if style.color.is_some() {
        base_style.color = style.color.unwrap();
    }


    if style.border_radius.is_some() {
        base_style.radius = get_radius(style.border_radius.clone().unwrap(),
                                "Rule".to_string()); 
    }

    let fillmode = 
        if style.fillmode_percent.is_some() {
            FillMode::Percent(style.fillmode_percent.unwrap())
        } else if style.fillmode_padded.is_some() {
            FillMode::Padded(style.fillmode_padded.unwrap())
        } else if style.fillmode_asymmetric_padding.is_some() {
            let a_padding = style.fillmode_asymmetric_padding.clone().unwrap();
            FillMode::AsymmetricPadding(a_padding[0], a_padding[1])
        } else {
            FillMode::Full
        };
    

    base_style.fill_mode = fillmode;

    base_style

}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgRuleStyleParam {
    IpgColor,
    RbgaColor,
    BorderRadius,
    FillModePercent,
    FillModePadded,
    FillModeAsymmetricPadding,
}

pub fn rule_style_update_item(style: &mut IpgRuleStyle,
                            item: PyObject,
                            value: PyObject,) {

    let update = try_extract_rule_style_update(item);
    match update {
        IpgRuleStyleParam::IpgColor => {
            let color = try_extract_ipg_color(value);
            style.color = get_color(None, Some(color), 1.0, false);
        },
        IpgRuleStyleParam::RbgaColor => {
            style.color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgRuleStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value));
        },
        IpgRuleStyleParam::FillModePercent => {
            style.fillmode_percent = Some(try_extract_f64(value) as f32);
        },
        IpgRuleStyleParam::FillModePadded => {
            style.fillmode_padded = Some(try_extract_u16(value))
        },
        IpgRuleStyleParam::FillModeAsymmetricPadding => {
            style.fillmode_asymmetric_padding = Some(try_extract_vec_u16(value))
        },
    }
}

pub fn try_extract_rule_style_update(update_obj: PyObject) -> IpgRuleStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgRuleStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Rule style update extraction failed"),
        }
    })
}
