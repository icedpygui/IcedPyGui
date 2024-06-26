//!Container container
#![allow(unused_assignments)]

use iced::{Color, Element, Length, Padding, Theme};
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
    pub style_color: Option<String>, 
    pub style_border: Option<String>, 
    pub style_shadow: Option<String>,
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
        style_color: Option<String>, 
        style_border: Option<String>, 
        style_shadow: Option<String>,
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
            style_color, 
            style_border, 
            style_shadow,
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
                .style(move|Theme|
                    get_styling(&Theme, 
                        con.style_color.clone(), 
                        con.style_border.clone(), 
                        con.style_shadow.clone(),
                        ))
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


pub fn get_styling(theme: &Theme,
                style_color: Option<String>, 
                style_border: Option<String>, 
                style_shadow: Option<String>,
                ) -> container::Style {
    
    let state = access_state();

    // Basically the default theme, using transparent to clarify
    let mut base_style = container::transparent(theme);

    let palette = theme.extended_palette();

    let color_palette_opt = if style_color.is_some() {
        state.styling_color.get(&style_color.unwrap())
    } else {
        None
    };

    if color_palette_opt.is_some() {

        let text_color = if palette.is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        };
        
        let color_palette = color_palette_opt.unwrap().clone();

        if color_palette.base.is_none() {
             base_style.background = None;
        } else {
            let color = color_palette.base.unwrap();
            if color.r == 0.123456 {
                base_style.background = Some(palette.background.weak.color.into()); 
            } else {
                base_style.background = Some(color.into());
            }
        }

        if color_palette.text.is_some() {
            base_style.text_color = Some(color_palette.text.unwrap());
        } else {
            base_style.text_color = Some(text_color);
        }
        
        if color_palette.border.is_some() {
            base_style.border.color = color_palette.border.unwrap();
        }

        if color_palette.shadow.is_some() {
            base_style.shadow.color = color_palette.shadow.unwrap();
        }
    }
    
    let border_opt = if style_border.is_some() {
        state.styling_border.get(&style_border.unwrap())
    } else {
        None
    };

    match border_opt {
        Some(bd) => {
            base_style.border.width = bd.width;
            base_style.border.radius = bd.radius;    
        },
        None => (),
    };


    let shadow_opt = if style_shadow.is_some() {
        state.styling_shadow.get(&style_shadow.unwrap())
    } else {
        None
    };

    match shadow_opt {
        Some(sh) => {
            base_style.shadow.offset = iced::Vector { x: sh.offset_x, y: sh.offset_y };
            base_style.shadow.blur_radius = sh.blur_radius;
        },
        None => (),
    };

   base_style

}