
use iced::{Padding, Length, Element};
use iced::alignment;
use iced::widget::{Column, Container};
use pyo3::pyclass;
use crate::app::Message;


#[derive(Debug)]
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
    // style: <Renderer::Theme as StyleSheet>::Style,
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
        // style: <Renderer::Theme as StyleSheet>::Style,
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
            // style,
        }
    }
}

pub fn construct_container(con: &IpgContainer, content: Vec<Element<'static, Message>> ) -> Element<'static, Message> {
    // iced container does not take a vec so need to put into a row or column first

    let col_content: Element<'static, Message> = Column::with_children(content)
                                                                        .width(Length::Shrink)
                                                                        .height(Length::Shrink)
                                                                        .into();

    let align_x = get_horizontal(con.align_x.clone());
    let align_y = get_vertical(con.align_y.clone());

    Container::new(col_content)
            .padding(10)
            .width(con.width)
            .height(con.height)
            .align_x(align_x)
            .align_y(align_y)
            .into()
}


#[derive(Debug, Clone)]
#[pyclass]
pub enum IpgContainerAlignment{
    Start,
    Center,
    End,
}


fn get_horizontal(x_align: IpgContainerAlignment) -> alignment::Horizontal {

    match x_align {
        IpgContainerAlignment::Start => alignment::Horizontal::Left,
        IpgContainerAlignment::Center => alignment::Horizontal::Center,
        IpgContainerAlignment::End => alignment::Horizontal::Right,
    }
}

fn get_vertical(y_align: IpgContainerAlignment) -> alignment::Vertical {
    
    match y_align {
        IpgContainerAlignment::Start => alignment::Vertical::Top,
        IpgContainerAlignment::Center => alignment::Vertical::Center,
        IpgContainerAlignment::End => alignment::Vertical::Bottom,
    }
}