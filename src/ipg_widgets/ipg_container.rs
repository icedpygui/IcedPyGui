//! ipg_container
use iced::{Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::widget::{container, horizontal_space, Container};
use pyo3::{pyclass, PyObject, Python};

use crate::app::Message;
use crate::graphics::colors::get_color;

use super::helpers::{get_height, get_horizontal_alignment, get_padding_f64, get_radius, get_vertical_alignment, get_width, try_extract_array_2, try_extract_boolean, try_extract_f64, try_extract_ipg_color, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_vec_f32, try_extract_vec_f64};
use super::ipg_enums::{IpgHorizontalAlignment, IpgVerticalAlignment, IpgWidgets};


#[derive(Debug, Clone)]
pub struct IpgContainer {
    pub id: usize,
    pub show: bool,

    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub align_x: IpgHorizontalAlignment,
    pub align_y: IpgVerticalAlignment,
    pub clip: bool,
    pub style_id: Option<usize>, 
}

impl IpgContainer {
    pub fn new(
        id: usize,
        show: bool,
        padding: Padding,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        align_x: IpgHorizontalAlignment,
        align_y: IpgVerticalAlignment,
        clip: bool,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            show,
            padding,
            width,
            height,
            max_width,
            max_height,
            align_x,
            align_y,
            clip,
            style_id, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgContainerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: [f32; 2],
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
}

impl IpgContainerStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        border_color: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow: Option<Color>,
        shadow_offset_xy: [f32; 2],
        shadow_blur_radius: f32,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            border_color,
            border_radius,
            border_width,
            shadow_color: shadow,
            shadow_offset_xy,
            shadow_blur_radius,
            text_color,
        }
    }
}


pub fn construct_container(con: IpgContainer, 
                            mut content: Vec<Element<Message>>,
                            style_opt: Option<IpgWidgets> ) 
                            -> Element<Message> {
    
    if !con.show {return horizontal_space().into()}

    let align_h = get_horizontal_alignment(con.align_x);
    let align_v = get_vertical_alignment(con.align_y);
    let style = get_cont_style(style_opt);

    // Since a container can have only one element and the 
    // the process sends a vec then if empty container, put in a
    // space or remove the element in the vec.
    let new_content: Element<Message> = if content.is_empty() {
        horizontal_space().into()
    } else {
        content.remove(0)
    };

    Container::new(new_content)
                .padding(con.padding)
                .width(con.width)
                .height(con.height)
                .align_x(align_h)
                .align_y(align_v)
                .clip(con.clip)
                .style(move|theme|
                    get_styling(theme, 
                        style.clone(),
                        ))
                .into()
    
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgContainerParam {
    AlignX,
    AlignY,
    Centered,
    Padding,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Clip,
    Show,
}

pub fn container_item_update(cont: &mut IpgContainer,
                            item: PyObject,
                            value: PyObject,
                            )
{
    let update = try_extract_container_update(item);
    let name = "Container".to_string();
    match update {
        IpgContainerParam::AlignX => {
            cont.align_x = try_extract_ipg_horizontal_alignment(value).unwrap();
        },
        IpgContainerParam::AlignY => {
            cont.align_y = try_extract_ipg_vertical_alignment(value).unwrap();
        },
        IpgContainerParam::Centered => {
            let centered = try_extract_boolean(value, name);
            if centered {
                cont.align_x = IpgHorizontalAlignment::Center;
                cont.align_y = IpgVerticalAlignment::Center;
            } else {
                cont.align_x = IpgHorizontalAlignment::Left;
                cont.align_y = IpgVerticalAlignment::Top;
            }
        },
        IpgContainerParam::Padding => {
            let pad = try_extract_vec_f64(value, name);
            cont.padding = get_padding_f64(pad);
        },
        IpgContainerParam::Width => {
            let w = Some(try_extract_f64(value, name) as f32);
            cont.width = get_width(w, false)
        },
        IpgContainerParam::WidthFill => {
            cont.width = get_width(None, try_extract_boolean(value, name));
        },
        IpgContainerParam::Height => {
            let h = Some(try_extract_f64(value, name) as f32);
            cont.height = get_height(h, false)
        },
        IpgContainerParam::HeightFill => {
            cont.height = get_height(None, try_extract_boolean(value, name));
        },
        IpgContainerParam::Clip => {
            cont.clip = try_extract_boolean(value, name);
        },
        IpgContainerParam::Show => {
            cont.show = try_extract_boolean(value, name);
        }
    }
}

pub fn try_extract_container_update(update_obj: PyObject) -> IpgContainerParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgContainerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Container update extraction failed"),
        }
    })
}

pub fn get_cont_style(style: Option<IpgWidgets>) -> Option<IpgContainerStyle>{
    match style {
        Some(st) => {
            match st {
                IpgWidgets::IpgContainerStyle(style) => {
                    Some(style)
                }
                _ => None,
            }
        },
        None => None,
    }
}

pub fn get_styling(theme: &Theme,
                style_opt: Option<IpgContainerStyle>,  
                ) -> container::Style {
    
    if style_opt.is_none() {
        return container::transparent(theme);
    }

    let style = style_opt.unwrap();

    let background_color = if style.background_color.is_some() {
        style.background_color.unwrap()
    } else {
        Color::TRANSPARENT
    };

    let mut border = Border::default();
    let mut shadow = Shadow::default();

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }

    border.radius = get_radius(style.border_radius.clone(), "Container".to_string());
    
    border.width = style.border_width;
    
    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();
        shadow.blur_radius = style.shadow_blur_radius;
        shadow.offset = Vector{ x: style.shadow_offset_xy[0], y: style.shadow_offset_xy[1] }
    }

    container::Style {
        background: Some(background_color.into()),
        border,
        shadow,
        text_color: style.text_color,
    }
    
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgContainerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

pub fn container_style_update_item(style: &mut IpgContainerStyle,
                            item: PyObject,
                            value: PyObject,) 
{
    let update = try_extract_container_style_update(item);
    let name = "ContainerStyle".to_string();
    match update {
        IpgContainerStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgContainerStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgContainerStyleParam::BorderRadius => {
            style.border_radius = try_extract_vec_f32(value, name);
        },
        IpgContainerStyleParam::BorderWidth => {
            style.border_width = try_extract_f64(value, name) as f32;
        },
        IpgContainerStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgContainerStyleParam::ShadowOffsetXY => {
            style.shadow_offset_xy = try_extract_array_2(value, name);
        },
        IpgContainerStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = try_extract_f64(value, name) as f32;
        },
        IpgContainerStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }

}

pub fn try_extract_container_style_update(update_obj: PyObject) -> IpgContainerStyleParam {

    Python::with_gil(|py| {
        let res = update_obj.extract::<IpgContainerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Container style parameter update extraction failed"),
        }
    })
}