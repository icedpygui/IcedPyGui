
use iced::{Padding, Length, Element};
use iced::alignment;
use iced::widget::{Column, Container};
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
    pub align_x: alignment::Horizontal,
    pub align_y: alignment::Vertical,
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
        align_x: alignment::Horizontal,
        align_y: alignment::Vertical,
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

    Container::new(col_content)
            .padding(10)
            .width(con.width)
            .height(con.height)
            .align_x(con.align_x)
            .align_y(con.align_y)
            .into()
}
