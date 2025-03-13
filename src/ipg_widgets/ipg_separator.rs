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
    pub parent_id: String,
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
        parent_id: String,
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
            parent_id,
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
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorType {
    Dot,
    Label,
    Line,
}

pub fn construct_separator<'a>(sep: &'a IpgSeparator, 
                            style_opt: Option<&IpgWidgets>) 
                            -> Option<Element<'a, app::Message>> {

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
    
    let separator: Element<'a, app::Message>  = if style_opt.is_some() {
        let style = style_opt.unwrap();

        sep_color = if style.color.is_some() {
            style.color.unwrap().into()
        } else {
            sep_color
        };

        border.color = if style.border_color.is_some() {
            style.border_color.unwrap()
        } else {
            border.color
        };

        match sep.separator_type {
            IpgSeparatorType::Dot => {  
                get_dot(sep, sep_color, border)
            },
            IpgSeparatorType::Label => {
                get_label(sep, sep_color)
            },
            IpgSeparatorType::Line => {
                get_line(sep, sep_color)
            },
        }
    } else {
        match sep.separator_type {
            IpgSeparatorType::Dot => {
                get_dot(sep, sep_color, border)
            },
            IpgSeparatorType::Label => {
                get_label(sep, sep_color)
            },
            IpgSeparatorType::Line => {
                get_line(sep, sep_color)
            },
        }
    };

    Some(separator)
    
}

fn get_dot<'a>(sep: &'a IpgSeparator, 
            sep_color: Color,
            border: Border) 
            -> Element<'a, app::Message>{
    
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
}

fn get_label<'a>(sep: &'a IpgSeparator,
            sep_color: Color) 
            -> Element<'a, app::Message> {

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

    let lbl = match &sep.label {
        Some(lbl) => lbl,
        None => panic!("Separator: A label is required for IpgSeparatorType::Label.")
    };

    Row::with_children(vec![
                        q_1, 
                        Text::new(lbl).color(sep_color).into(),
                        q_2,
                        ])
                        .spacing(sep.spacing)
                        .into()
}

fn get_line<'a>(sep: &'a IpgSeparator,
            sep_color: Color) 
            -> Element<'a, app::Message> {
    Quad {
        width: sep.width,
        height: sep.height,
        ..separator(sep_color.into())
    }.into()
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorParam {
    DotCount,
    DotFill,
    DotBorderWidth,
    DotRadius,
    Height,
    HeightFill,
    Label,
    Spacing,
    Show,
    StyleId,
    Width,
    WidthFill,
}


pub fn separator_item_update(sep: &mut IpgSeparator,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_separator_update(item);
    let name = "Separator".to_string();
    match update {
        IpgSeparatorParam::DotBorderWidth => {
            sep.dot_border_width = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::DotCount => {
            sep.dot_count = try_extract_i64(value, name) as usize;
        },
        IpgSeparatorParam::DotFill => {
            sep.dot_fill = try_extract_boolean(value, name);
        },
        IpgSeparatorParam::DotRadius => {
            sep.dot_radius = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::Label => {
            sep.label = Some(try_extract_string(value, name));
        },
        IpgSeparatorParam::Height => {
            let val = try_extract_f64(value, name);
            sep.height = get_height(Some(val as f32), false);
        },
        IpgSeparatorParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            sep.height = get_height(None, val);
        },
        IpgSeparatorParam::Spacing => {
            sep.spacing = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::Show => {
            sep.show = try_extract_boolean(value, name);
        },
        IpgSeparatorParam::StyleId => {
            sep.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgSeparatorParam::Width => {
            let val = try_extract_f64(value, name);
            sep.width = get_width(Some(val as f32), false);
        },
        IpgSeparatorParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            sep.width = get_width(None, val);
        },
    }

}

fn try_extract_separator_update(update_obj: &PyObject) -> IpgSeparatorParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSeparatorParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator update extraction failed"),
        }
    })
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorStyleParam {
    IpgColor,
    RbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
}

pub fn separator_style_update_item(style: &mut IpgSeparatorStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_separator_style_update(item);
    let name = "SeparatorStyle".to_string();
    match update {
        IpgSeparatorStyleParam::IpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::RbgaColor => {
            style.color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSeparatorStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn try_extract_separator_style_update(update_obj: &PyObject) -> IpgSeparatorStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgSeparatorStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator type update extraction failed"),
        }
    })
}

pub fn get_sep_style(style: Option<&IpgWidgets>) -> Option<IpgSeparatorStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgSeparatorStyle(style) => {
                    Some(style.clone())
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

