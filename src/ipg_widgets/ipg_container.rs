//!Container container
#![allow(unused_assignments)]
use iced::border::Radius;
use iced::{Background, Border, Color, Element, Length, Padding, Theme};
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

    let background_opt = if style_background.is_some() {
        state.styling_background.get(&style_background.unwrap())
    } else {
        None
    };
    
    let mut bg_color = Color::TRANSPARENT;
    let mut background = Background::Color(bg_color);

    match background_opt {
        Some(bg) => {
            bg_color = bg.color;
        },
        None => (),
    }

    background = Background::Color(bg_color);


    let mut border_color = Color::TRANSPARENT;
    let mut radius = Radius::from([0.0; 4]);
    let mut border_width = 1.0;

    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    match border_opt {
        Some(bd) => {
            border_color = bd.color;
            radius = bd.radius;
            border_width = bd.width;
        },
        None => (),
    }

    let border = Border{ color: border_color, width: border_width, radius };


    let mut shadow_color: Color = Color::TRANSPARENT;
    let mut offset_x: f32 = 0.0;
    let mut offset_y: f32 = 0.0;
    let mut blur_radius: f32 = 0.0;

    let shadow_opt = if style_shadow.is_some() {
        state.styling_shadow.get(&style_shadow.unwrap())
    } else {
        None
    };

    match shadow_opt {
        Some(sh) => {
            shadow_color = sh.color;
            offset_x = sh.offset_x;
            offset_y = sh.offset_y;
            blur_radius =sh.blur_radius;
        },
        None => (),
    }

    let shadow = iced::Shadow { color: shadow_color, offset: 
                                        iced::Vector { x: offset_x, y: offset_y }, 
                                        blur_radius };

    let text_color_opt = if style_text_color.is_some() {
        state.styling_text_color.get(&style_text_color.unwrap())
    } else {
        None
    };
    
    let mut text_color = None;


    match text_color_opt {
        Some(tc) => {
            text_color = Some(tc.color);
        },
        None => (),
    }

    container::Style {
            background: Some(background),
            border,
            shadow,
            text_color,
            }

}