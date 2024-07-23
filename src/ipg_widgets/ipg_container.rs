//!Container container
#![allow(unused_assignments)]

use iced::border::Radius;
use iced::theme::palette::Pair;
use iced::{Background, Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::alignment;
use iced::widget::{container, Column, Container};


use crate::access_state;
use crate::app::Message;
use crate::style::styling::{is_dark, IpgColorPalette};

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
    pub align_x: IpgHorizontalAlignment,
    pub align_y: IpgVerticalAlignment,
    pub center_xy: bool,
    pub clip: bool,
    pub style: Option<String>, 
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
        center_xy: bool,
        clip: bool,
        style: Option<String>,
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
            style, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgContainerStyle {
    pub id: usize,
    pub base: Option<Color>,
    pub strong: Option<Color>,
    pub weak: Option<Color>,
    pub strong_factor: Option<f32>,
    pub weak_factor: Option<f32>,
    pub border: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow: Option<Color>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub text: Option<Color>,
    pub use_background: bool,
}

impl IpgContainerStyle {
    pub fn new(
        id: usize,
        base: Option<Color>,
        strong: Option<Color>,
        weak: Option<Color>,
        strong_factor: Option<f32>,
        weak_factor: Option<f32>,
        border: Option<Color>,
        border_radius: Vec<f32>,
        border_width: f32,
        shadow: Option<Color>,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        shadow_blur_radius: f32,
        text: Option<Color>,
        use_background: bool,
    ) -> Self {
        Self {
            id,
            base,
            strong,
            weak,
            strong_factor,
            weak_factor,
            border,
            border_radius,
            border_width,
            shadow,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text,
            use_background,
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
                        con.style.clone(),
                        ))
                .into();
    cont.into()
}


fn get_horizontal(x_align: IpgHorizontalAlignment, center: bool) -> alignment::Horizontal {

    if center {return alignment::Horizontal::Center}

    match x_align {
        IpgHorizontalAlignment::Left => alignment::Horizontal::Left,
        IpgHorizontalAlignment::Center => alignment::Horizontal::Center,
        IpgHorizontalAlignment::Right => alignment::Horizontal::Right,
    }
}

fn get_vertical(y_align: IpgVerticalAlignment, center: bool) -> alignment::Vertical {
    
    if center {return alignment::Vertical::Center}

    match y_align {
        IpgVerticalAlignment::Top => alignment::Vertical::Top,
        IpgVerticalAlignment::Center => alignment::Vertical::Center,
        IpgVerticalAlignment::Bottom => alignment::Vertical::Bottom,
    }
}


pub fn get_styling(theme: &Theme,
                style: Option<String>,  
                ) -> container::Style {
    
    let state = access_state();

    if style.is_none() {
        return container::transparent(theme);
    }

    let style_opt = if style.is_some() {
        state.container_style.get(&style.unwrap())
    } else {
        panic!("Container style: Could not find container style id")
    };

    let style = if style_opt.is_some() {
        style_opt.unwrap()
    } else {
        panic!("Container style: style id not found.")
    };

    if style.base.is_none() && style.weak.is_some() {
        panic!("Container style: if you define style.weak, you must define style.base too")
    }

    // Strong is not used in container
    let mut base = Color::default();
    let mut weak = Color::default();

    let palette = theme.extended_palette();

    let mut background = Some(palette.background.weak.color.into());
    let background_color = theme.palette().background;

    let mut text_color = None;
    
    let mut border = Border::default();
    let mut shadow = Shadow::default();

    // all custom colors
    if style.base.is_some() && style.weak.is_some() {
       base = style.base.unwrap();
       weak = style.weak.unwrap();
       text_color = Some(get_text_color(style.text, weak));
    }

    // generate weak
    if style.base.is_some() && style.weak.is_none() {
        if !style.use_background {
            base = style.base.unwrap();
            text_color = Some(get_text_color(style.text, base));
            let custom = IpgColorPalette::generate(base, background_color, 
                                                            text_color.unwrap(), 
                                                            style.strong_factor, style.weak_factor);

            background = Some(Background::Color(custom.weak.color));
        }
    }

    if style.border.is_some() {
        border.color = style.border.unwrap();
    }

    if style.border_radius.len() == 1 {
        border.radius = Radius::from(style.border_radius[0]);
    } else if style.border_radius.len() == 4 {
        let radius = [style.border_radius[0], style.border_radius[1], 
                                style.border_radius[2], style.border_radius[3]];
        border.radius = Radius::from(radius);
    } else {
        panic!("Container style: Border radius must be a list of 1 or 4 items")
    }
    
    border.width = style.border_width;
    
    if style.shadow.is_some() {
        shadow.color = style.shadow.unwrap();
        shadow.blur_radius = style.shadow_blur_radius;
        shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y }
    }
    
    container::Style {
        background,
        border,
        shadow,
        text_color,
    }
    
}

fn get_text_color(text: Option<Color>, weak: Color) -> Color {
    if text.is_some() {
        text.unwrap()
   } else {
        let mut t_color = Color::BLACK;
        if is_dark(weak) {
            t_color = Color::WHITE;
        } 
        let pair = Pair::new(weak, t_color);
        pair.text
   }
}