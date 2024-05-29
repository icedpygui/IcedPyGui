//!Container container
use iced::{Element, Length, Padding};
use iced::alignment;
use iced::widget::{Column, Container};
use iced::widget::container::{transparent, Catalog, Style, StyleFn};

use pyo3::pyclass;

use crate::app::Message;
use crate::style::styling::get_container_styling;

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
        align_x: IpgContainerAlignment,
        align_y: IpgContainerAlignment,
        center_xy: bool,
        clip: bool,
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
            style_id: None,
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
                .style(move|Theme|get_container_styling(&Theme, con.style_id.clone()))
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

#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgContainerTheme {
    Default,
    Custom,
}

impl Catalog for IpgContainerTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}
