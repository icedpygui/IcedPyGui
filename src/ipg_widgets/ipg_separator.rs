//! ipg_separator

use crate::app::Message;
use crate::graphics::colors::{get_color, IpgColor};
use crate::iced_aw_widgets::menu::quad::{InnerBounds, Quad};

use crate::app;
use super::helpers::{get_height, get_width, 
    try_extract_boolean, try_extract_f64, try_extract_i64, 
    try_extract_ipg_color, try_extract_rgba_color, 
    try_extract_string};
use super::ipg_enums::IpgWidgets;

use iced::border::Radius;


use pyo3::{pyclass, PyObject, Python};

use iced::widget::{row, Row, Text};
use iced::{Background, Border, Color, Element, 
    Length, Renderer, Theme };


#[derive(Debug, Clone)]
pub struct IpgSeparator {
    pub id: usize,
    pub separator_type: IpgSeparatorType,
    pub label: Option<String>,
    pub label_left_width: f32,
    pub label_right_width: f32,
    pub dot_radius: f32,
    pub dot_count: usize,
    pub dot_fill: bool,
    pub dot_border_width: f32,
    pub width: Length,
    pub height: Length,
    pub spacing: f32,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl IpgSeparator {
    pub fn new( 
        id: usize,
        separator_type: IpgSeparatorType,
        label: Option<String>,
        label_left_width: f32,
        label_right_width: f32,
        dot_radius: f32,
        dot_count: usize,
        dot_fill: bool,
        dot_border_width: f32,
        width: Length,
        height: Length,
        spacing: f32,
        style_id: Option<usize>,
        show: bool,
        ) -> Self {
        Self {
            id,
            separator_type,
            label,
            label_left_width,
            label_right_width,
            dot_radius,
            dot_count,
            dot_fill,
            dot_border_width,
            width,
            height,
            spacing,
            style_id,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgSeparatorStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub border_color: Option<Color>,
}

impl IpgSeparatorStyle {
    pub fn new(
        id: usize,
        color: Option<Color>,
        border_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            color,
            border_color,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass]
pub enum IpgSeparatorType {
    Dot,
    Label,
    Line,
}

pub fn construct_separator(sep: IpgSeparator, 
                            style_opt: Option<IpgWidgets>) 
                            -> Option<Element<'static, app::Message>> {

    if !sep.show {
        return None
    }

    let style_opt = get_sep_style(style_opt);

    let mut sep_color = get_color(
                                    None, 
                                    Some(IpgColor::PRIMARY), 
                                    1.0, 
                                    false).unwrap();
    let mut border = Border::default();
    
    let separator: Element<'static, app::Message>  = if style_opt.is_some() {
        let style = style_opt.unwrap();

        sep_color = if style.color.is_some() {
            style.color.unwrap().into()
        } else {
            get_color(
                None, 
                Some(IpgColor::PRIMARY), 
                1.0, 
                false).unwrap()
        };

        border.color = if style.border_color.is_some() {
            style.border_color.unwrap()
        } else {
            border.color
        };

        match sep.separator_type {
            IpgSeparatorType::Dot => {  
                let color = if sep.dot_fill {
                    sep_color
                } else {
                    Color::TRANSPARENT
                };
                let radius = sep.dot_radius;
                // Shrink doesn't seem to work so sub in radius
                let width = if sep.width == Length::Shrink {
                    Length::Fixed(radius*2.0)
                } else {
                    sep.width
                };
                
                let height = if sep.height == Length::Shrink {
                    Length::Fixed(radius*2.0)
                } else {
                    sep.height
                };
                dbg!(width, height);
                row((0..sep.dot_count).map(|_| {
                    Quad {
                        inner_bounds: InnerBounds::Square(radius * 2.0),
                        quad_border: Border {
                            radius: Radius::new(radius),
                            color: border.color,
                            width: sep.dot_border_width,
                            ..Default::default()
                        },
                        width,
                        height,
                        quad_color: color.into(),
                        ..Default::default()
                    }.into()
                }))
                .height(height)
                .spacing(sep.spacing)
                .into()
            },
            IpgSeparatorType::Label => {
                let q_1: Element<Message, Theme, Renderer> = Quad {
                    width: Length::Fixed(sep.label_left_width),
                    ..separator(sep_color.into())
                }.into();
                let q_2: Element<Message, Theme, Renderer> = Quad {
                    width: Length::Fixed(sep.label_right_width),
                    ..separator(sep_color.into())
                }.into();
            
                let lbl = match sep.label {
                    Some(lbl) => lbl,
                    None => panic!("Separator: A label is required for IpgSeparatorType::Label.")
                };
            
                Row::with_children(vec![
                                    q_1, 
                                    Text::new(lbl).into(),
                                    q_2,
                                    ])
                                    .spacing(sep.spacing)
                                    .into()
            },
            IpgSeparatorType::Line => {
                Quad {
                    ..separator(sep_color.into())
                }.into()
            },
        }
    } else {
    
        match sep.separator_type {
            IpgSeparatorType::Dot => {
                let color = if sep.dot_fill {
                    sep_color
                } else {
                    Color::TRANSPARENT
                };
                row((0..sep.dot_count).map(|_| {
                    default_quad(sep.separator_type.clone(), 
                            color.into(),
                            sep.width,
                            sep.height,
                            sep.dot_radius).into()
                }))
                .height(20.0)
                .spacing(sep.spacing)
                .into()
            },
            IpgSeparatorType::Label => {
                let q_1: Element<Message, Theme, Renderer> = Quad {
                    width: Length::Fixed(sep.label_left_width),
                    height: sep.height,
                    ..separator(sep_color.into())
                }.into();
                let q_2: Element<Message, Theme, Renderer> = Quad {
                    width: Length::Fixed(sep.label_right_width),
                    height: sep.height,
                    ..separator(sep_color.into())
                }.into();
            
                let lbl = match sep.label {
                    Some(lbl) => lbl,
                    None => panic!("Separator: A label is required for IpgSeparatorType::Label.")
                };
            
                Row::with_children(vec![
                                    q_1, 
                                    Text::new(lbl).into(),
                                    q_2,
                                    ])
                                    .spacing(sep.spacing)
                                    .into()
            },
            IpgSeparatorType::Line => {
                Quad {
                    width: sep.width,
                    height: sep.height,
                    ..separator(sep_color.into())
                }.into()
            },
        }
        
    };

    Some(separator)
    
}



#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSeparatorParam {
    DotCount,
    DotFill,
    DotBorderWidth,
    DotRadius,
    Height,
    HeightFill,
    Label,
    SeparatorType,
    Spacing,
    Show,
    StyleId,
    Width,
    WidthFill,
}


pub fn separator_item_update(sep: &mut IpgSeparator,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_separator_update(item);

    match update {
        IpgSeparatorParam::DotBorderWidth => {
            sep.dot_border_width = try_extract_f64(value) as f32;
        },
        IpgSeparatorParam::DotCount => {
            sep.dot_count = try_extract_i64(value) as usize;
        },
        IpgSeparatorParam::DotFill => {
            sep.dot_fill = try_extract_boolean(value);
        },
        IpgSeparatorParam::DotRadius => {
            sep.dot_radius = try_extract_f64(value) as f32;
        },
        IpgSeparatorParam::Label => {
            sep.label = Some(try_extract_string(value));
        },
        IpgSeparatorParam::Height => {
            let val = try_extract_f64(value);
            sep.height = get_height(Some(val as f32), false);
        },
        IpgSeparatorParam::HeightFill => {
            let val = try_extract_boolean(value);
            sep.height = get_height(None, val);
        },
        IpgSeparatorParam::SeparatorType => {
            sep.separator_type = try_extract_sep_type(value);
        },
        IpgSeparatorParam::Spacing => {
            sep.spacing = try_extract_f64(value) as f32;
        },
        IpgSeparatorParam::Show => {
            sep.show = try_extract_boolean(value);
        },
        IpgSeparatorParam::StyleId => {
            sep.style_id = Some(try_extract_f64(value) as usize);
        },
        IpgSeparatorParam::Width => {
            let val = try_extract_f64(value);
            sep.width = get_width(Some(val as f32), false);
        },
        IpgSeparatorParam::WidthFill => {
            let val = try_extract_boolean(value);
            sep.width = get_width(None, val);
        },
    }

}

fn try_extract_separator_update(update_obj: PyObject) -> IpgSeparatorParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSeparatorParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator update extraction failed"),
        }
    })
}

fn try_extract_sep_type(update_obj: PyObject) -> IpgSeparatorType {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSeparatorType>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator type update extraction failed"),
        }
    })
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgSeparatorStyleParam {
    IpgColor,
    RbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
}

pub fn separator_style_update_item(style: &mut IpgSeparatorStyle,
                            item: PyObject,
                            value: PyObject,) 
{
    let update = try_extract_separator_style_update(item);
    match update {
        IpgSeparatorStyleParam::IpgColor => {
            let color = try_extract_ipg_color(value);
            style.color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::RbgaColor => {
            style.color = Some(Color::from(try_extract_rgba_color(value)));
        },
        IpgSeparatorStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value)));
        },
    }
}

fn try_extract_separator_style_update(update_obj: PyObject) -> IpgSeparatorStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSeparatorStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator type update extraction failed"),
        }
    })
}

pub fn get_sep_style(style: Option<IpgWidgets>) -> Option<IpgSeparatorStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgSeparatorStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}

fn separator(bg_color: Background) -> Quad {
    Quad {
        quad_color: bg_color,
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        width: Length::Shrink,
        height: Length::Shrink,
        ..Default::default()
    }
}

fn default_quad(quad_type: IpgSeparatorType, 
                quad_color: Color, 
                width: Length,
                height: Length,
                dot_radius: f32) 
-> Quad
{
    match quad_type {
        IpgSeparatorType::Dot => {
            let radius = dot_radius;
            let width = if width == Length::Shrink {
                Length::Fixed(radius*2.0)
            } else {
                width
            };
            let height = if height == Length::Shrink {
                Length::Fixed(radius*2.0)
            } else {
                height
            };
            Quad {
                inner_bounds: InnerBounds::Square(radius * 2.0),
                quad_border: Border {
                    radius: Radius::new(radius),
                    color: quad_color,
                    width: 2.0,
                    ..Default::default()
                },
                width,
                height,
                quad_color: quad_color.into(),
                ..Default::default()
            }
        },
        IpgSeparatorType::Label => {
            Quad {
                width,
                height,
                ..separator(quad_color.into())
            }
        },
        IpgSeparatorType::Line => {
            Quad {
                ..separator(quad_color.into())
            }
        },
    }
}
