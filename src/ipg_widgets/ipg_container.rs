//! ipg_container
use iced::{Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::widget::{container, horizontal_space, Container};

use crate::app::Message;

use super::helpers::{get_horizontal_alignment, get_radius, get_vertical_alignment};
use super::ipg_enums::{IpgHorizontalAlignment, IpgVerticalAlignment};


#[derive(Debug, Clone)]
pub struct IpgContainer {
    pub id: usize,
    pub show: bool,

    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub align_h: Option<IpgHorizontalAlignment>,
    pub align_v: Option<IpgVerticalAlignment>,
    pub clip: bool,
    pub style_id: Option<String>, 
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
        align_h: Option<IpgHorizontalAlignment>,
        align_v: Option<IpgVerticalAlignment>,
        clip: bool,
        style_id: Option<String>,
    ) -> Self {
        Self {
            id,
            show,
            padding,
            width,
            height,
            max_width,
            max_height,
            align_h,
            align_v,
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
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
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
        shadow_offset_x: f32,
        shadow_offset_y: f32,
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
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
        }
    }
}


pub fn construct_container<'a>(con: IpgContainer, 
                            mut content: Vec<Element<'a, Message>>,
                            style: Option<IpgContainerStyle> ) 
                            -> Element<'a, Message> {

    let align_h = get_horizontal_alignment(con.align_h.clone());
    let align_v = get_vertical_alignment(con.align_v.clone());

    let mut new_content: Element<Message> = horizontal_space().into();
    if content.len() > 0 {
        new_content = content.remove(0);
    }

    let cont: Element<Message> = Container::new(new_content)
                .padding(con.padding)
                .width(con.width)
                .height(con.height)
                .align_x(align_h)
                .align_y(align_v)
                .clip(con.clip)
                .style(move|theme|
                    get_styling(&theme, 
                        style.clone(),
                        ))
                .into();
    cont.into()
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
        shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y }
    }

    container::Style {
        background: Some(background_color.into()),
        border,
        shadow,
        text_color: style.text_color,
    }
    
}

