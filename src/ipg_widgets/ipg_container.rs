//!Container container
#![allow(unused_assignments)]

use iced::{Background, Border, Element, Length, Padding, Shadow, Theme};
use iced::alignment;
use iced::widget::{container, Column, Container};


use pyo3::pyclass;

use crate::access_state;
use crate::app::Message;


#[derive(Debug, Clone)]
pub struct IpgContainer {
    pub id: usize,
    pub show: bool,

    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub align_x: IpgContainerAlignment,
    pub align_y: IpgContainerAlignment,
    pub center_xy: bool,
    pub clip: bool,
    pub style_background: Option<String>, 
    pub style_border: Option<String>, 
    pub style_shadow: Option<String>,
    pub style_text_color: Option<String>,
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
        align_x: IpgContainerAlignment,
        align_y: IpgContainerAlignment,
        center_xy: bool,
        clip: bool,
        style_background: Option<String>, 
        style_border: Option<String>, 
        style_shadow: Option<String>,
        style_text_color: Option<String>
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
            center_xy,
            clip,
            style_background, 
            style_border, 
            style_shadow,
            style_text_color
        }
    }
}

pub fn construct_container(con: IpgContainer, content: Vec<Element<'static, Message>> ) -> Element<'static, Message> {
    // iced container does not take a vec so need to put into a row or column first
    let col_content: Element<'static, Message> = Column::with_children(content)
                                                                        .width(Length::Shrink)
                                                                        .height(Length::Shrink)
                                                                        .into();

    let align_x = get_horizontal(con.align_x.clone(), con.center_xy);
    let align_y = get_vertical(con.align_y.clone(), con.center_xy);

    

    let cont: Element<Message> = Container::new(col_content)
                .padding(con.padding)
                .width(con.width)
                .height(con.height)
                .align_x(align_x)
                .align_y(align_y)
                .clip(con.clip)
                .style(move|Theme|get_styling(&Theme, 
                                                        con.style_background.clone(), 
                                                        con.style_border.clone(), 
                                                        con.style_shadow.clone(),
                                                        con.style_text_color.clone()))
                .into();
    cont.into()
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgContainerAlignment{
    Start,
    Center,
    End,
}


fn get_horizontal(x_align: IpgContainerAlignment, center: bool) -> alignment::Horizontal {

    if center {return alignment::Horizontal::Center}

    match x_align {
        IpgContainerAlignment::Start => alignment::Horizontal::Left,
        IpgContainerAlignment::Center => alignment::Horizontal::Center,
        IpgContainerAlignment::End => alignment::Horizontal::Right,
    }
}

fn get_vertical(y_align: IpgContainerAlignment, center: bool) -> alignment::Vertical {
    
    if center {return alignment::Vertical::Center}

    match y_align {
        IpgContainerAlignment::Start => alignment::Vertical::Top,
        IpgContainerAlignment::Center => alignment::Vertical::Center,
        IpgContainerAlignment::End => alignment::Vertical::Bottom,
    }
}


pub fn get_styling(_theme: &Theme,
                style_background: Option<String>, 
                style_border: Option<String>, 
                style_shadow: Option<String>,
                style_text_color: Option<String>) 
                -> container::Style {
    
    let state = access_state();

    let default_style = container::Style::default();

    let background_opt = if style_background.is_some() {
        state.styling_background.get(&style_background.unwrap())
    } else {
        None
    };
    

   let background =  match background_opt {
        Some(bg) => {
            Some(Background::Color(bg.color))
        },
        None => default_style.background,
    };


    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    let border = match border_opt {
        Some(bd) => {
            Border {
                color: bd.color,
                width: bd.width,
                radius: bd.radius,
            }
        },
        None => default_style.border,
    };


    let shadow_opt = if style_shadow.is_some() {
        state.styling_shadow.get(&style_shadow.unwrap())
    } else {
        None
    };

    let shadow: Shadow = match shadow_opt {
        Some(sh) => {
            Shadow {
                color: sh.color,
                offset: iced::Vector { x: sh.offset_x, y: sh.offset_y },
                blur_radius: sh.blur_radius,
            }
        },
        None => default_style.shadow,
    };

    let text_color_opt = if style_text_color.is_some() {
        state.styling_text_color.get(&style_text_color.unwrap())
    } else {
        None
    };

    let text_color = match text_color_opt {
        Some(tc) => {
            Some(tc.color)
        },
        None => default_style.text_color,
    };

    container::Style {
            background,
            border,
            shadow,
            text_color,
            }

}